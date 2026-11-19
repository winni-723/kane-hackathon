# 病歷去識別化網站（Kane CLI Hackathon）

Kane CLI Online Hackathon 參賽專案。交件截止：**2026-11-27（五）13:30 PST**（台灣時間 11/28（六）05:30）。原本是 11/19，主辦於 2026-11-18 延後。截止後不能再推送修正。

## 專案

使用者貼上病歷文字，網站遮蔽個資（姓名、電話、地址、生日、家屬名字等），保留醫療資訊（症狀、診斷、用藥），並可下載結果。

## 技術架構

- **後端**：Rust + Axum，負責去識別化引擎。
- **前端**：TypeScript + Vite，build 後由 Axum 直接提供靜態檔案。整個專案只部署一個服務。
- **引擎在瀏覽器裡執行**（2026-11-18 起）：同一份 Rust 引擎以 `--features wasm` 編成 WebAssembly（`src/wasm.rs`、`bash scripts/build-wasm.sh` → `web/src/engine-wasm/`，不進 git），頁面在 Web Worker 裡執行（`web/src/engine-worker.ts`、`web/src/engine.ts`）。伺服器只提供檔案，沒有接收病歷的路由。今天的日期由頁面傳入。需要 `rustup target add wasm32-unknown-unknown` 與版本和 `Cargo.lock` 一致的 `wasm-bindgen-cli`（目前 0.2.130）。第一層的 `scripts/wasm-matches-native.mjs` 要求 102 份測試輸入在瀏覽器版與原生版的輸出完全相同。改了 `src/` 之後，`tsc` 用的是上次建置的 `engine-wasm`；`verify.sh` 會重建。
- **病歷語言**：英文，美國格式（美國電話號碼、SSN、MM/DD/YYYY 日期、美國地址與 ZIP code、MRN）。引擎規則與測試資料都以此為準。
- **測試資料**：AI 產生的假病歷，每份附一份答案卷（JSON），放在 `tests/fixtures/`。答案卷列出「應遮蔽的文字」與「應保留的文字」。
- **Kane 測試**：`tests/kane/*_test.md`，由答案卷產生。commit 測試檔與 `output-<stem>/Result.md`；`.internal/` 錄製留在本機。
- **頁面功能**（2026-11-19 改為上傳優先）：最上面是上傳區（選檔或拖放 `.txt`、`.docx`、`.csv`、`.pdf`），下面一行說明病歷只在瀏覽器裡處理；接著是唯一的範例選單「Or try a fictional sample」＋「Load sample file」，6 個選項：Progress note (text)、Three records (text)，以及 4 個虛構檔（放在 `web/public/samples/`，與測試檔逐位元組相同，由 `pdf_test.rs` 的 `page_samples_are_the_test_files` 把關，走與選檔相同的上傳流程）；最後是「Or paste one record」與 Process。結果頁有 Result、Download result、Edit original、Record details（遮蔽摘要與擷取欄位，放在 Result 區之外）。多份病歷（Three records、多選檔案或 CSV）產生 Research table：View 打開一份，Previous / Next 切換，Download CSV、Download all records（所有去識別化病歷存成一個 `.txt`）。Load sample record、Load sample batch 兩個按鈕已拿掉。Kane 錄製以角色與名稱找元素，不看位置，所以調整版面不必重錄；改按鈕或標籤名稱則要。

## 雙層驗證閉環

每完成一個功能，依序執行：

1. **第一層（靜態與單元）**：`cargo build`、`cargo clippy`、`cargo test`，以及前端的 `tsc`。
2. **第二層（Kane CLI）**：跑完整流程（貼上病歷 → 處理 → 檢查畫面），依答案卷確認「應遮蔽的文字都不在畫面上、應保留的文字都還在」。
3. **任何一層失敗**：讀取錯誤訊息，修正程式，再**從第一層重跑**。
4. **兩層都通過才 commit**。commit 訊息要寫明通過了哪些測試。

### 怎麼跑

```bash
bash scripts/verify.sh
```

一次跑完兩層，第一層失敗就停。exit 0 才代表兩層都通過；exit 2 代表 Kane 沒有啟動（多半是登入過期），不算測試失敗。腳本會自己關掉、重編、重啟本機伺服器（Windows 上執行中的執行檔會擋住重編）。

- 遮蔽率統計表：`cargo test --test fixtures_test scoreboard -- --nocapture`
- 第一層的答案卷測試在 `tests/fixtures_test.rs`。`medical_content_is_always_kept` 對全部 20 份病歷檢查 `must_keep`；`gated_pii_is_removed` 檢查 `GATES` 列出的（個資類型, 難度）必須 100% 遮蔽。**`GATES` 只增不減。**

### 自動 hook

`.claude/settings.json` 註冊了兩個 hook，閉環不需要手動觸發：

- **PostToolUse（Write|Edit）** → `scripts/hooks/post-edit.mjs`：改到 `src/`、`tests/fixtures_test.rs`、`Cargo.toml` 時跑 `cargo test`；改到 `web/src/`、`web/index.html` 時跑 `tsc`。失敗會以錯誤回報失敗的測試；通過會回報三組資料的分數。用 `target-dev` 建置，不會動到執行中的伺服器。
- **Stop** → `scripts/hooks/stop-verify.mjs`：引擎、頁面或 Kane 測試自上次完整驗證後有變動時，跑 `bash scripts/verify.sh`。測試失敗會擋下結束並回報 Kane 的說明；Kane 沒有啟動則放行並警告。狀態記在 `.kane-logs/verified.json`。

hook 回報失敗時，修程式，不要改答案卷或 Kane 測試，也不要為了分數保留會誤遮醫療資訊的規則。

### 三組資料

| 資料夾 | 用途 | 是否把關 |
|---|---|---|
| `tests/fixtures`（20 份） | 開發集 | 是，`GATES` 要求 100% |
| `tests/heldout`（10 份手寫） | 測驗卷，已用於調校 | 否，只量測 |
| `tests/synthea`（58 份，Synthea v4.0.0） | s01–s29 可用於調校；**s30–s58 不可逐筆檢視**，只看總分 | 否，只量測 |

- 測驗卷的答案卷在引擎第一次跑之前就由使用者定案，同樣不可修改。
- 第一次未調校的分數（2026-11-14）：手寫 62/88、Synthea 151/537。這是泛化能力的誠實數字，寫在 README，不要覆蓋。
- 看分數：`cargo test --test fixtures_test -- --nocapture scoreboard`
- 已知且刻意不修的漏網：全小寫的姓名、只由一般介系詞引出的姓名（`for Leilani Kahananui`）。修它們會誤遮醫療用語。

### 研究數據表（擷取）

- `src/extract.rs` 從每份病歷擷取年齡、性別、疾病、用藥。除了「由生日算年齡」之外，一律從**去識別化後的文字**擷取，表格裡不可能出現被遮蔽的個資。
- 擷取答案卷在 `tests/extraction/`（20 份，使用者 2026-11-14 定案，規則見該資料夾的 README）。同樣不可為了讓測試通過而修改。
- 第一層測試在 `tests/extraction_test.rs`：`GATES` 列出必須全對的（欄位, 難度），只增不減；`must_not_list` 的字串與任何應遮蔽的個資都不得出現在表格裡。看分數：`cargo test --test extraction_test -- --nocapture`。
- 由生日算的年齡以測試中的 `REFERENCE_DAY`（2026-11-14）為基準，不要改這個日期。
- 抓不到就留空，不要為了分數去猜（例如由姓名猜性別）。
- 已知且刻意不修的漏網：hard-05 的性別（代名詞一比一）、hard-06 的 `COPD`（只由 `has` 引出）。
- 測驗卷的擷取答案卷在 `tests/extraction-heldout/`（10 份，使用者 2026-11-15 定案），只量測、不把關。第一次未調校的分數是 **25/40**，寫在 README 與該資料夾的 README，不要覆蓋；調校後 37/40。
- 測驗卷刻意不修的漏網：hard-101 的 `tonic-clonic seizure`、hard-102 的 `Contact dermatitis`（沒有引導詞的單句）、medium-103 的 `H. pylori`（藥物後面的 `for`）。修它們會把症狀與用藥原因當成診斷。
- Synthea 沒有擷取答案卷，其分段格式（`MEDICATIONS:`、`CONDITIONS:`）目前不讀。

### 文件上傳（Word，之後是 CSV、PDF）

使用者 2026-11-16 決定：要貼近真實產品，支援 `.docx`、CSV、PDF（有文字層），遮蔽標準採 HIPAA Safe Harbor。掃描檔 OCR、HL7/FHIR、DICOM 不做，寫在 README 的 Limits。

- `src/document.rs` 把上傳的文件讀成純文字（頁首、內文、註腳、頁尾；表格一列一行、儲存格以 tab 分隔），之後走同一個 `engine::process`。檔案在瀏覽器裡由引擎讀取（`read_file`），**不傳檔名、也不傳檔案**。
- 測試集在 `tests/documents/`：`source/<id>.json`（內容與版面）→ `python scripts/gen-docx.py` → `<id>.docx`；答案卷在 `keys/<id>.json`。測試文件必須有真實版面（抬頭、頁尾重複姓名、表格、簽名區），不可只是既有病歷轉檔。
- 答案卷比 `tests/fixtures` 多了 `facility_name`、`facility_address`、`facility_phone`、`provider_name`、`service_date`、`other_id`，全部應遮蔽。答案卷同樣不可為了讓測試通過而修改。
- 第一層測試在 `tests/documents_test.rs`：`reading_a_document_loses_nothing`（答案卷的每個字串都要在讀出的文字裡）、`medical_content_is_always_kept`、`gated_identifiers_are_removed`（`GATES` 只增不減）。看分數：`cargo test --test documents_test scoreboard -- --nocapture`。
- 第一次未調校的分數（2026-11-16，6 份 Word）：**98/132，醫療資訊 81/81**。寫在 README 與 `tests/documents/README.md`，不要覆蓋；調校後 132/132。
- CSV（2026-11-16）：`document::records_from_csv` 把每一列轉成一份「`欄位名: 值`」的病歷，值原樣保留（不在讀取時改寫）。答案卷多一個 `table`，列出每一列在研究數據表應顯示的年齡、性別、診斷、用藥；`csv_table_scoreboard` 要求全對。第一次分數：個資 82/82、醫療 31/31、表格 37/40（調校後 40/40），不要覆蓋。`read_file` 回傳 `{ records: [...] }`。
- 研究數據表在 Word 文件上沒有量測，表格儲存格裡的診斷與用藥大多抓不到。
- 測試資料的 SSN 一律用從未發出的號段（`987-65-43xx`）。`docx-06` 原本誤用 `512-55-0199`，使用者 2026-11-16 指示改為 `987-65-4320`。
- 這些是開發集，目前沒有文件的測驗卷。檔案上傳由第一層把關；Kane 的上傳測試（2026-11-18，4 個，`tests/kane/upload-*_test.md`）由 `node scripts/gen-kane-upload-test.mjs <docx-03|csv-01|pdf-01|pdf-06>` 產生，從頁面的「Load sample file」選單選檔（不用檔案選擇器），錄製用 `bash scripts/author-kane-tests.sh upload:<id>`。Kane 套件共 188 個測試（頁面本身 5 個：ui-download、ui-menu-record、ui-menu-batch、ui-menu-details、ui-batch-view）。
- `.testmuai/evidence/` 在 OneDrive 資料夾裡，Kane 結束時改名證據檔偶爾會被鎖住（EPERM），測試全過但沒有 `testrun_done`、exit 1。這不是測試失敗，重跑 `verify.sh` 即可（不扣 credits）。
- 既有的 88 份病歷、答案卷與 183 個 Kane 測試不因新增格式而修改。
- PDF（2026-11-17）：測試集在 `tests/documents/pdf/`（`source/` → `python scripts/gen-pdf.py`，需要 Chrome、Word、PyMuPDF；`keys/`）。pdf-01、04 由 Chrome 列印，02、05 由 Word 另存，03 是固定位置的傳真表單，06 是 03 的掃描檔（沒有答案卷，必須回傳 `DocumentError::Scanned`）。答案卷使用者 2026-11-17 同意，不可修改。第一層測試在 `tests/pdf_test.rs`（讀取不可遺漏、醫療資訊全保留、掃描檔被拒），看分數：`cargo test --test pdf_test scoreboard -- --nocapture`。
- PDF 第一次分數（不要覆蓋）：套件原樣 81/100、讀壞 3、醫療 59/62；修好讀取程式、未改引擎 88/100、醫療 62/62。調校後 100/100、醫療 62/62，`tests/pdf_test.rs` 的 `GATES` 涵蓋所有（類型, 難度），只增不減。
- `vendor/pdf-extract` 是 pdf-extract 0.12.1 加一個修正（字寬表範圍格式讀錯，造成 Chrome PDF 字中間多空格），經 `[patch.crates-io]` 使用，說明在 `vendor/README.md`。
- 使用者 2026-11-17 指示：規則修正依主題或細項分開 commit，訊息寫清楚改了什麼，不要全部修完一次 commit；每一步開始前先說明打算做什麼。

### 新增一種個資的流程

1. 改 `src/engine.rs`，在其單元測試加上該類型的格式與「不該誤遮」的案例。
2. 看統計表，把達到 100% 的（類型, 難度）加進 `GATES`。
3. 用 `node scripts/gen-kane-test.mjs <record_id> <pii_type>` 由答案卷產生 Kane 測試。**全部涵蓋**（使用者 2026-11-14 決定）：已進 `GATES` 的（類型, 難度）底下，每一份含該類個資的病歷都要有一個 Kane 測試。腳本拒絕覆寫既有測試。
   - 還沒進 `GATES` 的組合（引擎還抓不到的困難題）不建立 Kane 測試，否則一建立就失敗，會擋住所有 commit。
4. 建立錄製：多個新測試用 `kane-cli testrun run <files> --agent --headless --parallel 3`（官方支援的平行方式，已驗證不會互相污染）；單一測試用 `kane-cli testmd run <file> --agent --headless`。之後確認答案卷的檢查（contains / does not contain the text）都是 `exact`。出現 `VISUAL` 的測試要刪掉 `output-<stem>/` 重新錄製。「Result heading is visible」這類導覽檢查用截圖判讀沒關係。
5. `bash scripts/verify.sh` 全過後才 commit。

**錄製期間不可同時跑驗證**（2026-11-15 的教訓，多花約 630 credits）：Stop hook 在 Kane 測試檔變動時會自動跑 `verify.sh`；當時錄製腳本沒有上鎖，驗證與錄製同時進行，驗證把還沒有錄製檔的 64 個新測試又錄了一遍，並重啟伺服器讓 14 個錄製失敗。現在的保護：

- `scripts/kane-lock.sh`：錄製腳本與 `verify.sh` 共用 `.kane-logs/verify.lock`，同一時間只有一個在跑。
- `verify.sh` 遇到沒有錄製檔（`output-<stem>/.internal`）的測試會以 exit 2 停下，不會自己錄製。驗證永遠不該花 credits。
- 不要繞過腳本直接用 `kane-cli testrun run` 錄製新測試。

常用腳本：

- `bash scripts/author-kane-tests.sh <record>:<type> ...`（或 `--from <清單檔>`）：產生並錄製新測試，結束時列出需要重錄的（`REDO`）。超過約 15 個測試要用背景執行，前景上限 10 分鐘。
- `bash scripts/author-kane-tests.sh hand:tests/kane/<name>_test.md`：錄製手寫的測試（例如頁面測試）。檔案要先寫好；已有錄製的會略過。
- `bash scripts/redo-kane-recordings.sh <file> ...`：重錄尚未 commit 的測試；已 commit 的會被拒絕。實測約每 8 個新測試有 1 個需要重錄。
- `node scripts/cleanup-kane-chrome.mjs`：清掉 Kane 殘留的背景 Chrome。症狀是測試在 0 秒內結束、錯誤訊息為 `All CDP ports 9222-9230 are in use`。只清 `kane-clean-*` 設定檔的程序，不動使用者自己的 Chrome。
- `cargo run --example show -- <record_id>`：印出某份病歷的去識別化結果，用來肉眼檢查。
- 伺服器執行中（例如背景正在錄製 Kane 測試）時要跑測試，用 `CARGO_TARGET_DIR=target-dev cargo test`，避免去動被鎖住的執行檔。

Kane 測試在 commit 之後即凍結。尚未 commit 的新測試若檢查方式不可信（例如出現 `VISUAL`），可以刪掉重新產生；已 commit 的不行。

## 絕對規則

- **不可以為了讓測試通過而修改或刪除答案卷、既有的 Kane 測試。**
- **只能修改程式碼來讓測試通過。**
- **每次跑 Kane 時，要把之前所有的測試一起重跑**，確認沒有改壞。
- **所有測試資料都必須是虛構的**，不可以使用任何真實個資。

如果認為某份答案卷或某個既有 Kane 測試本身是錯的，停下來告訴使用者，由使用者決定，不可自行修改。

## Kane CLI 使用方式

以下取自官方的 agent 說明（`testmuai.com/kane-cli-agent/AGENTS.md`）與本機 `--help`，版本 0.10.4。

### 執行

- 一律加 `--agent`（輸出 NDJSON）與 `--headless`，並給足夠的 timeout。一次執行約 30 秒到數分鐘。
- 每個指令前標記 runtime：PowerShell 用 `$env:KANE_CLI_USER_AGENT='claude-code'; kane-cli ...`。
- 每次執行都需要起始 URL（`--url` 或寫在 objective 開頭）。本機 app 可直接用 `http://127.0.0.1:<port>`。
- 單一測試：`kane-cli testmd run <path>_test.md --agent --headless`，結束事件是 `test_md_done`。
- 整組測試：`kane-cli testrun run --agent --headless`，結束事件是 `testrun_done`。兩個以上的測試用這個，不要自己寫迴圈跑 `testmd run`。
- **回歸執行一律加 `--no-adaptive-heal`。** 預設情況下重播對不上畫面時 Kane 會自動重寫測試，這等同於改動既有測試，違反絕對規則。

### 判讀結果

先看 exit code，再解析輸出：

| Exit code | 意義 |
|---|---|
| `0` | 通過 |
| `1` | 失敗：流程跑了但沒通過 |
| `2` | 沒有啟動（登入、設定、Chrome、網路）。不算測試失敗，也不扣 credits |
| `3` | 逾時或被取消 |

`run_end` 事件中可用的欄位：`status`、`summary`、`one_liner`、`reason`、`credits_consumed`、`final_state`、`test_url`。失敗時讀 `output-<stem>/Result.md` 找出失敗的步驟。

### 撰寫 objective

- 每個流程以「畫面上看得到什麼」的斷言結尾（verify / assert）。
- 斷言結果，不要斷言觸發條件。
- 要取值時寫 `store X as 'name'`。
- 單一 objective 控制在約 15 步內。
- 不可寫需要中途輸入的步驟。

### 檢查方法（步驟 0 實驗結論，2026-11-13）

以下每一條都用對照組驗證過（故意讓不該出現的文字出現，確認 Kane 會判定失敗）。

- **絕對不要同時平行執行多個 `kane-cli run`。** 它們會共用同一個瀏覽器，互相把文字打進對方的頁面，結果完全不可信。一次只跑一個；要平行就用 `kane-cli testrun run --parallel`。
- **檢查一律限定在 Result 區**，寫成 `verify that the Result region contains the text '...'` 與 `verify that the Result region does not contain the text '...'`。這樣 Kane 會對 Result 區做精確的 DOM 字串比對（`innerText.includes(...)`）。
- **每個流程至少要有一個「存在」檢查搭配「不存在」檢查。** 如果文字沒有成功輸入或 Kane 看錯區域，「存在」檢查會先失敗。
- **病歷文字用變數檔傳入**（`--variables-file`，objective 裡寫 `{{record}}`），不要寫在 objective 字串裡。多行文字已驗證可原樣輸入。
- **每次執行後用 `node scripts/kane-summary.mjs <log>` 看 Kane 實際檢查了哪個元素。** `locators` 應該是 Result 區；若是別的元素或 `analyzer=visual` 且沒有 locator，該次結果不可信，要重寫 objective。
- **「包含」與「不包含」的檢查要放在同一個步驟裡。** 步驟裡只有「不包含」檢查時，Kane 會改用截圖判讀；和「包含」檢查放一起時，兩者都是精確比對。`gen-kane-test.mjs` 已照此產生。
- **含表格的區塊（例如 Record details）要寫成「the Record details region, taken as a whole, contains / does not contain the text '...'」**（2026-11-19）。只寫 region 時 Kane 會對準單一儲存格，「不包含」就變成截圖判讀；ui-menu-details 錄了三次才確認。
- **不同區塊的檢查放在不同步驟。** 同一步驟混合 Result 與 Record details 的檢查時，Kane 找不準區塊（ui-batch-view）。
- **存檔測試的重播已驗證**（2026-11-14）：3 個測試重播約 40 秒、不扣 credits；暫時關掉電話遮蔽後，三個測試在 `--no-adaptive-heal` 下全部失敗。重播抓得到退步。
- `tests/kane/output-*/.internal/` 是本機的重播錄製，內含 TestMu 帳號與專案 id，已列入 `.gitignore`。遺失的話重新建立錄製即可，建立一個測試實際約 25–35 credits（2026-11-14 以 12 個測試量得平均 34）。
- 頁面設計配合檢查：輸入區與 Result 區同一時間只顯示一個，錯誤訊息元素只在出錯時才存在。不要加入空白的佔位元素，Kane 曾把空的錯誤訊息元素誤認為 Result 區。

### Credits

- 第一次執行會「撰寫」每個步驟並扣 credits；之後未變動的步驟以錄製重播，不扣 credits。
- 修改第 N 步的文字會讓第 N 步及其後所有步驟重新撰寫。這是另一個不要改動既有測試的理由。
- 用 `kane-cli balance` 檢查餘額。

### 不要 commit

- `.testmuai/evidence/`
- 任何含有憑證的檔案

## 登入

- 登入由使用者自己執行（`kane-cli login --oauth`）。不要在對話中索取或回顯 access key、密碼、token。
- OAuth token 曾在隔天過期。執行前用 `kane-cli whoami` 確認；若顯示未登入，請使用者重新登入。

## Hackathon 交件要求

- Repo 必須在 2026-11-12 當天或之後初始化（主辦會查 commit 紀錄），公開或邀請評審。
- README 要有安裝步驟。
- App 要已部署，或本地一行指令可跑。評審要能在 30 秒內看到 app 運作。
- 3 分鐘內的 demo 影片，要展示 app 運作與 Kane 執行。
- 評分四項等權重：Ships、Verified、Closed loop、Craft。平手先比 Verified，再比 Closed loop。
