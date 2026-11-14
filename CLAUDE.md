# 病歷去識別化網站（Kane CLI Hackathon）

Kane CLI Online Hackathon 參賽專案。交件截止：**2026-11-19 23:59 IST**（台灣時間 11/20 02:29）。截止後不能再推送修正。

## 專案

使用者貼上病歷文字，網站遮蔽個資（姓名、電話、地址、生日、家屬名字等），保留醫療資訊（症狀、診斷、用藥），並可下載結果。

## 技術架構

- **後端**：Rust + Axum，負責去識別化引擎。
- **前端**：TypeScript + Vite，build 後由 Axum 直接提供靜態檔案。整個專案只部署一個服務。
- **病歷語言**：英文，美國格式（美國電話號碼、SSN、MM/DD/YYYY 日期、美國地址與 ZIP code、MRN）。引擎規則與測試資料都以此為準。
- **測試資料**：AI 產生的假病歷，每份附一份答案卷（JSON），放在 `tests/fixtures/`。答案卷列出「應遮蔽的文字」與「應保留的文字」。
- **Kane 測試**：`*_test.md` 檔案，連同旁邊的 `output-<stem>/` 錄製資料夾一起 commit。

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

### 新增一種個資的流程

1. 改 `src/engine.rs`，在其單元測試加上該類型的格式與「不該誤遮」的案例。
2. 看統計表，把達到 100% 的（類型, 難度）加進 `GATES`。
3. 用 `node scripts/gen-kane-test.mjs <record_id> <pii_type>` 由答案卷產生 Kane 測試（每個 gate 至少兩份病歷）。腳本拒絕覆寫既有測試。
4. 逐一用 `kane-cli testmd run <file> --agent --headless` 建立錄製（一次一個，不可平行），再用 `node scripts/kane-testmd-summary.mjs <log>` 確認每個檢查都是 `exact`。出現 `VISUAL` 就不可信。
5. `bash scripts/verify.sh` 全過後才 commit。

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
- **存檔測試的重播已驗證**（2026-11-14）：3 個測試重播約 40 秒、不扣 credits；暫時關掉電話遮蔽後，三個測試在 `--no-adaptive-heal` 下全部失敗。重播抓得到退步。
- `tests/kane/output-*/.internal/` 是本機的重播錄製，內含 TestMu 帳號與專案 id，已列入 `.gitignore`。遺失的話重新建立錄製即可，每個測試約 20 credits。
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
