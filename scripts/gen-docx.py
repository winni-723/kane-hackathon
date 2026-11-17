"""Build the .docx test documents from their sources.

    python scripts/gen-docx.py

Reads every tests/documents/source/*.json and writes tests/documents/<id>.docx.
Only the Python standard library is used: a .docx file is a zip of XML parts.

A source file describes a document the way a real one is laid out: a page
header and footer (repeated on every page), headings, paragraphs and tables.
All content is fictional.
"""
import json
import sys
import zipfile
from pathlib import Path
from xml.sax.saxutils import escape

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "tests" / "documents" / "source"
OUT = ROOT / "tests" / "documents"

W = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
R = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"


def run(text, bold=False, size=None):
    props = ""
    if bold:
        props += "<w:b/>"
    if size:
        props += f'<w:sz w:val="{size}"/>'
    if props:
        props = f"<w:rPr>{props}</w:rPr>"
    return f'<w:r>{props}<w:t xml:space="preserve">{escape(text)}</w:t></w:r>'


def page_field():
    return '<w:fldSimple w:instr=" PAGE "><w:r><w:t>1</w:t></w:r></w:fldSimple>'


def paragraph(text, bold=False, size=None, align=None):
    props = f'<w:pPr><w:jc w:val="{align}"/></w:pPr>' if align else ""
    # "{PAGE}" in a footer line becomes a live page-number field.
    parts = text.split("{PAGE}")
    runs = page_field().join(run(p, bold, size) if p else "" for p in parts)
    return f"<w:p>{props}{runs}</w:p>"


def table(rows, header_row=False):
    border = "".join(
        f'<w:{side} w:val="single" w:sz="4" w:space="0" w:color="808080"/>'
        for side in ("top", "left", "bottom", "right", "insideH", "insideV")
    )
    out = [f'<w:tbl><w:tblPr><w:tblW w:w="5000" w:type="pct"/><w:tblBorders>{border}</w:tblBorders></w:tblPr>']
    for i, row in enumerate(rows):
        cells = "".join(
            f"<w:tc>{paragraph(cell, bold=header_row and i == 0)}</w:tc>" for cell in row
        )
        out.append(f"<w:tr>{cells}</w:tr>")
    out.append("</w:tbl>")
    return "".join(out)


def body(blocks):
    out = []
    for block in blocks:
        if "title" in block:
            out.append(paragraph(block["title"], bold=True, size=32, align="center"))
        elif "h" in block:
            out.append(paragraph(block["h"], bold=True, size=24))
        elif "p" in block:
            out.append(paragraph(block["p"]))
        elif "table" in block:
            out.append(table(block["table"], block.get("header_row", False)))
            out.append("<w:p/>")
        else:
            sys.exit(f"unknown block: {block}")
    return "".join(out)


def part(tag, inner):
    return f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:{tag} xmlns:w="{W}" xmlns:r="{R}">{inner}</w:{tag}>'


CONTENT_TYPES = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
    '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
    '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
    '<Default Extension="xml" ContentType="application/xml"/>'
    '<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>'
    '<Override PartName="/word/header1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/>'
    '<Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/>'
    "</Types>"
)
RELS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
    '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
    '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>'
    "</Relationships>"
)
DOC_RELS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
    '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
    '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/>'
    '<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/>'
    "</Relationships>"
)
SECTION = (
    '<w:sectPr><w:headerReference w:type="default" r:id="rId1"/>'
    '<w:footerReference w:type="default" r:id="rId2"/>'
    '<w:pgSz w:w="12240" w:h="15840"/>'
    '<w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>'
    "</w:sectPr>"
)

def build(doc, target):
    """Writes one source document as a .docx file."""
    header = "".join(
        paragraph(line, bold=(i == 0), size=24 if i == 0 else 16, align="center")
        for i, line in enumerate(doc["header"])
    )
    footer = "".join(paragraph(line, size=16) for line in doc["footer"])
    parts = [
        ("[Content_Types].xml", CONTENT_TYPES),
        ("_rels/.rels", RELS),
        ("word/_rels/document.xml.rels", DOC_RELS),
        ("word/document.xml", part("document", f"<w:body>{body(doc['body'])}{SECTION}</w:body>")),
        ("word/header1.xml", part("hdr", header)),
        ("word/footer1.xml", part("ftr", footer)),
    ]
    with zipfile.ZipFile(target, "w", zipfile.ZIP_DEFLATED) as z:
        for name, content in parts:
            # A fixed timestamp, so rebuilding an unchanged source gives the
            # same bytes and git shows no difference.
            info = zipfile.ZipInfo(name, date_time=(2026, 11, 16, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            z.writestr(info, content)


if __name__ == "__main__":
    for source in sorted(SOURCE.glob("*.json")):
        doc = json.loads(source.read_text(encoding="utf-8"))
        target = OUT / f"{doc['id']}.docx"
        build(doc, target)
        print(f"wrote {target.relative_to(ROOT)}")
