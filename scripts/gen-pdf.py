"""Build the PDF test documents from their sources.

    python scripts/gen-pdf.py

Reads every tests/documents/pdf/source/*.json and writes
tests/documents/pdf/<id>.pdf. Real PDFs come from different programs, and each
lays text out its own way, so each source names the program that makes it:

  chrome  An HTML page printed by Chrome, the way a patient portal or an EHR
          screen is printed. The footer repeats on every page.
  word    A Word document (built as scripts/gen-docx.py builds one) saved as
          PDF by Microsoft Word. Long paragraphs wrap wherever Word decides.
  form    Text placed at fixed positions with PyMuPDF, the way form and fax
          software writes a page: fields in two columns.
  scan    Another test PDF turned into a picture, as if it had been printed and
          scanned. The file holds an image and no text at all.

Needs Chrome, Microsoft Word and PyMuPDF (pip install pymupdf). PDFs made by
Chrome and Word carry their own timestamps, so a rebuild is not byte-identical.
All content is fictional.
"""
import html
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import fitz  # PyMuPDF

ROOT = Path(__file__).resolve().parent.parent
PDF_DIR = ROOT / "tests" / "documents" / "pdf"
SOURCE = PDF_DIR / "source"

CHROME_PATHS = [
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/usr/bin/google-chrome",
]


def load_gen_docx():
    spec = importlib.util.spec_from_file_location("gen_docx", ROOT / "scripts" / "gen-docx.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# ---------------------------------------------------------------- chrome

CSS = """
@page { size: letter; margin: 0.6in; }
body { font-family: Arial, Helvetica, sans-serif; font-size: 10.5pt; color: #111; }
.letterhead { border-bottom: 2px solid #1d4e7a; padding-bottom: 6px; margin-bottom: 10px; }
.letterhead .org { font-size: 15pt; font-weight: bold; color: #1d4e7a; }
.letterhead .sub { font-size: 8.5pt; color: #444; }
h1 { font-size: 13pt; text-align: center; letter-spacing: 1px; margin: 8px 0 12px; }
h2 { font-size: 11pt; background: #e8eef4; padding: 3px 6px; margin: 14px 0 6px; }
table { border-collapse: collapse; width: 100%; }
td, th { border: 1px solid #9aa; padding: 3px 6px; text-align: left; vertical-align: top; }
th { background: #f2f2f2; }
table.demo td:nth-child(odd) { font-weight: bold; background: #f7f7f7; width: 16%; }
p { margin: 6px 0; line-height: 1.35; }
.footer { position: fixed; bottom: 0; left: 0; right: 0; font-size: 8pt; background: #fff;
          color: #333; border-top: 1px solid #999; padding-top: 3px; }
.break { break-before: page; }
"""


def chrome_html(doc):
    e = html.escape
    out = ["<!doctype html><html><head><meta charset='utf-8'><style>", CSS, "</style></head><body>"]
    out.append("<div class='letterhead'>")
    for i, line in enumerate(doc["header"]):
        out.append(f"<div class='{'org' if i == 0 else 'sub'}'>{e(line)}</div>")
    out.append("</div>")
    out.append("<div class='footer'>" + "<br>".join(e(l) for l in doc["footer"]) + "</div>")
    for block in doc["body"]:
        if "title" in block:
            out.append(f"<h1>{e(block['title'])}</h1>")
        elif "h" in block:
            out.append(f"<h2>{e(block['h'])}</h2>")
        elif "p" in block:
            out.append(f"<p>{e(block['p'])}</p>")
        elif "pagebreak" in block:
            out.append("<div class='break'></div>")
        elif "table" in block:
            header = block.get("header_row", False)
            out.append("<table>" if header else "<table class='demo'>")
            for i, row in enumerate(block["table"]):
                tag = "th" if header and i == 0 else "td"
                out.append("<tr>" + "".join(f"<{tag}>{e(c)}</{tag}>" for c in row) + "</tr>")
            out.append("</table>")
        else:
            sys.exit(f"unknown block: {block}")
    out.append("</body></html>")
    return "".join(out)


def build_chrome(doc, target, tmp):
    chrome = next((p for p in CHROME_PATHS if os.path.exists(p)), None)
    if not chrome:
        sys.exit("Chrome not found")
    page = Path(tmp) / f"{doc['id']}.html"
    page.write_text(chrome_html(doc), encoding="utf-8")
    subprocess.run(
        [chrome, "--headless=new", "--disable-gpu", "--no-pdf-header-footer",
         f"--user-data-dir={Path(tmp) / 'chrome-profile'}",
         f"--print-to-pdf={target}", page.as_uri()],
        check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=120,
    )


# ---------------------------------------------------------------- word

def build_word(doc, target, tmp):
    docx = Path(tmp) / f"{doc['id']}.docx"
    load_gen_docx().build(doc, docx)
    # Word is driven through PowerShell, so no Python COM package is needed.
    script = (
        "$w = New-Object -ComObject Word.Application; $w.Visible = $false; "
        f"$d = $w.Documents.Open('{docx}', $false, $true); "
        f"$d.SaveAs2('{target}', 17); $d.Close($false); $w.Quit()"
    )
    subprocess.run(["powershell", "-NoProfile", "-Command", script], check=True, timeout=180)


# ---------------------------------------------------------------- form

def build_form(doc, target):
    pdf = fitz.open()
    page = pdf.new_page(width=612, height=792)
    for line in doc["lines"]:
        font = "hebo" if line.get("bold") else "helv"
        page.insert_text((line["x"], line["y"]), line["t"], fontname=font, fontsize=line.get("size", 10))
    # Rules under the fax banner and each section title, as on a printed form.
    for y in (36, 139, 262, 334, 437):
        page.draw_line((36 if y == 36 else 72, y), (576 if y == 36 else 540, y), width=0.6)
    save_fixed(pdf, target)


def build_scan(doc, target):
    original = fitz.open(PDF_DIR / f"{doc['from']}.pdf")
    picture = original[0].get_pixmap(dpi=150, colorspace=fitz.csGRAY)
    pdf = fitz.open()
    page = pdf.new_page(width=612, height=792)
    page.insert_image(page.rect, stream=picture.tobytes("png"))
    save_fixed(pdf, target)


def save_fixed(pdf, target):
    # Fixed metadata and no random file id, so a rebuild gives the same bytes.
    pdf.set_metadata({"producer": "gen-pdf.py", "creationDate": "D:20261117000000", "modDate": "D:20261117000000"})
    pdf.save(target, garbage=3, deflate=True, no_new_id=True)


# ---------------------------------------------------------------- main

def main():
    sources = [json.loads(p.read_text(encoding="utf-8")) for p in sorted(SOURCE.glob("*.json"))]
    # A scan is made from another document, so it is built last.
    sources.sort(key=lambda d: d["producer"] == "scan")
    with tempfile.TemporaryDirectory() as tmp:
        for doc in sources:
            target = PDF_DIR / f"{doc['id']}.pdf"
            kind = doc["producer"]
            if kind == "chrome":
                build_chrome(doc, target, tmp)
            elif kind == "word":
                build_word(doc, target, tmp)
            elif kind == "form":
                build_form(doc, target)
            elif kind == "scan":
                build_scan(doc, target)
            else:
                sys.exit(f"{doc['id']}: unknown producer {kind}")
            print(f"wrote {target.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
