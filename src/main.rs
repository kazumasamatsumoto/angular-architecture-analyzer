use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub category: String,
    pub status: String,
    pub details: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct FrontendAnalyzer {
    pub project_path: String,
}

impl FrontendAnalyzer {
    pub fn new(project_path: String) -> Self {
        Self { project_path }
    }

    pub fn analyze_all(&self) -> Result<Vec<AnalysisResult>> {
        let mut results = Vec::new();

        results.push(self.analyze_ui_screens()?);
        results.push(self.analyze_data_storage()?);
        results.push(self.analyze_api()?);
        results.push(self.analyze_authentication()?);
        results.push(self.analyze_session_management()?);
        results.push(self.analyze_security()?);
        results.push(self.analyze_state_management()?);
        results.push(self.analyze_routing()?);
        results.push(self.analyze_ui_design_system()?);
        results.push(self.analyze_error_handling()?);
        results.push(self.analyze_performance()?);

        Ok(results)
    }

    // 画面解析
    pub fn analyze_ui_screens(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "画面".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        // HTMLファイルを検索
        let html_files = self.find_files_with_extension("html")?;
        result
            .details
            .push(format!("HTMLファイル数: {}", html_files.len()));

        // Angularコンポーネントファイルを検索
        let component_files = self.find_files_with_extension("component.ts")?;
        result.details.push(format!(
            "Angularコンポーネント数: {}",
            component_files.len()
        ));

        // CSSファイルを検索
        let css_files = self.find_files_with_extension("css")?;
        let scss_files = self.find_files_with_extension("scss")?;
        result.details.push(format!(
            "スタイルファイル数: {} (CSS: {}, SCSS: {})",
            css_files.len() + scss_files.len(),
            css_files.len(),
            scss_files.len()
        ));

        // レスポンシブ対応チェック
        if self.check_responsive_design(&css_files, &scss_files)? {
            result
                .details
                .push("レスポンシブデザイン: 実装済み".to_string());
        } else {
            result
                .warnings
                .push("レスポンシブデザインの実装が確認できません".to_string());
        }

        Ok(result)
    }

    // データ保持解析
    pub fn analyze_data_storage(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "データ保持".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut local_storage_usage = 0;
        let mut session_storage_usage = 0;
        let mut indexed_db_usage = 0;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("localStorage") {
                    local_storage_usage += 1;
                }
                if content.contains("sessionStorage") {
                    session_storage_usage += 1;
                }
                if content.contains("indexedDB") || content.contains("IndexedDB") {
                    indexed_db_usage += 1;
                }
            }
        }

        result
            .details
            .push(format!("localStorage使用箇所: {}", local_storage_usage));
        result
            .details
            .push(format!("sessionStorage使用箇所: {}", session_storage_usage));
        result
            .details
            .push(format!("IndexedDB使用箇所: {}", indexed_db_usage));

        if local_storage_usage == 0 && session_storage_usage == 0 && indexed_db_usage == 0 {
            result
                .warnings
                .push("データ保持機能が確認できません".to_string());
        }

        Ok(result)
    }

    // API解析
    pub fn analyze_api(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "API".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut http_client_usage = 0;
        let mut api_endpoints = Vec::new();
        let mut error_handling_count = 0;

        let url_regex = Regex::new(r#"["'](https?://[^"']+)["']"#).unwrap();
        let http_methods = ["get", "post", "put", "delete", "patch"];

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                // HTTP Clientの使用をチェック
                if content.contains("HttpClient") || content.contains("http.") {
                    http_client_usage += 1;
                }

                // APIエンドポイントを抽出
                for cap in url_regex.captures_iter(&content) {
                    api_endpoints.push(cap[1].to_string());
                }

                // HTTPメソッドの使用をチェック
                for method in &http_methods {
                    if content.contains(&format!(".{}(", method)) {
                        result
                            .details
                            .push(format!("HTTP {}メソッド使用確認", method.to_uppercase()));
                    }
                }

                // エラーハンドリングをチェック
                if content.contains("catchError") || content.contains("catch(") {
                    error_handling_count += 1;
                }
            }
        }

        result.details.push(format!(
            "HTTPクライアント使用ファイル数: {}",
            http_client_usage
        ));
        result.details.push(format!(
            "検出されたAPIエンドポイント数: {}",
            api_endpoints.len()
        ));
        result.details.push(format!(
            "エラーハンドリング実装箇所: {}",
            error_handling_count
        ));

        if http_client_usage == 0 {
            result
                .warnings
                .push("HTTP通信の実装が確認できません".to_string());
        }

        if error_handling_count == 0 {
            result
                .warnings
                .push("APIエラーハンドリングが確認できません".to_string());
        }

        Ok(result)
    }

    // 認証解析
    pub fn analyze_authentication(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "ログイン".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut auth_service_found = false;
        let mut login_component_found = false;
        let mut jwt_usage = false;
        let mut password_validation = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                // 認証サービスの存在チェック
                if content.contains("AuthService") || content.contains("auth.service") {
                    auth_service_found = true;
                }

                // ログインコンポーネントの存在チェック
                if content.contains("login") || content.contains("Login") {
                    login_component_found = true;
                }

                // JWT使用チェック
                if content.contains("jwt") || content.contains("JWT") || content.contains("token") {
                    jwt_usage = true;
                }

                // パスワードバリデーションチェック
                if content.contains("password")
                    && (content.contains("validate") || content.contains("required"))
                {
                    password_validation = true;
                }
            }
        }

        result.details.push(format!(
            "認証サービス: {}",
            if auth_service_found {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "ログイン機能: {}",
            if login_component_found {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "JWT/トークン認証: {}",
            if jwt_usage {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "パスワード検証: {}",
            if password_validation {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        if !auth_service_found {
            result
                .warnings
                .push("認証サービスが確認できません".to_string());
        }

        if !login_component_found {
            result
                .warnings
                .push("ログイン機能が確認できません".to_string());
        }

        Ok(result)
    }

    // セッション管理解析
    pub fn analyze_session_management(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "セッション管理".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut session_timeout = false;
        let mut auto_logout = false;
        let mut session_storage_usage = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("timeout") || content.contains("expire") {
                    session_timeout = true;
                }

                if content.contains("logout") && content.contains("auto") {
                    auto_logout = true;
                }

                if content.contains("sessionStorage") {
                    session_storage_usage = true;
                }
            }
        }

        result.details.push(format!(
            "セッションタイムアウト: {}",
            if session_timeout {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "自動ログアウト: {}",
            if auto_logout {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "セッションストレージ使用: {}",
            if session_storage_usage {
                "確認済み"
            } else {
                "未確認"
            }
        ));

        if !session_timeout {
            result
                .warnings
                .push("セッションタイムアウト機能が確認できません".to_string());
        }

        Ok(result)
    }

    // セキュリティ解析
    pub fn analyze_security(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "セキュリティ".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut sanitization_found = false;
        let mut csrf_protection = false;
        let mut https_enforcement = false;
        let mut dangerous_patterns = Vec::new();

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                // サニタイズ処理のチェック
                if content.contains("sanitize") || content.contains("DomSanitizer") {
                    sanitization_found = true;
                }

                // CSRF対策のチェック
                if content.contains("csrf") || content.contains("CSRF") {
                    csrf_protection = true;
                }

                // HTTPS強制のチェック
                if content.contains("https") && content.contains("redirect") {
                    https_enforcement = true;
                }

                // 危険なパターンのチェック
                if content.contains("innerHTML") {
                    dangerous_patterns
                        .push("innerHTML使用が検出されました（XSSリスクあり）".to_string());
                }

                if content.contains("eval(") {
                    dangerous_patterns.push(
                        "eval()関数の使用が検出されました（セキュリティリスクあり）".to_string(),
                    );
                }
            }
        }

        result.details.push(format!(
            "入力値サニタイズ: {}",
            if sanitization_found {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "CSRF対策: {}",
            if csrf_protection {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "HTTPS強制: {}",
            if https_enforcement {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        for pattern in dangerous_patterns {
            result.warnings.push(pattern);
        }

        if !sanitization_found {
            result
                .warnings
                .push("入力値のサニタイズ処理が確認できません".to_string());
        }

        Ok(result)
    }

    // 状態管理解析
    pub fn analyze_state_management(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "状態管理".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut ngrx_usage = false;
        let mut akita_usage = false;
        let mut service_usage = false;
        let mut subject_usage = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("@ngrx") || content.contains("NgRx") {
                    ngrx_usage = true;
                }

                if content.contains("akita") || content.contains("Akita") {
                    akita_usage = true;
                }

                if content.contains("@Injectable") {
                    service_usage = true;
                }

                if content.contains("Subject") || content.contains("BehaviorSubject") {
                    subject_usage = true;
                }
            }
        }

        result.details.push(format!(
            "NgRx使用: {}",
            if ngrx_usage {
                "確認済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "Akita使用: {}",
            if akita_usage {
                "確認済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "サービス実装: {}",
            if service_usage {
                "確認済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "Subject使用: {}",
            if subject_usage {
                "確認済み"
            } else {
                "未確認"
            }
        ));

        if !ngrx_usage && !akita_usage && !subject_usage {
            result
                .warnings
                .push("状態管理ライブラリまたはパターンが確認できません".to_string());
        }

        Ok(result)
    }

    // ルーティング解析
    pub fn analyze_routing(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "ルーティング・ナビゲーション".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut routing_module_found = false;
        let mut guards_found = false;
        let mut lazy_loading = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("RouterModule") || content.contains("Routes") {
                    routing_module_found = true;
                }

                if content.contains("CanActivate") || content.contains("Guard") {
                    guards_found = true;
                }

                if content.contains("loadChildren") {
                    lazy_loading = true;
                }
            }
        }

        result.details.push(format!(
            "ルーティング設定: {}",
            if routing_module_found {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "ガード機能: {}",
            if guards_found {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "遅延読み込み: {}",
            if lazy_loading {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        if !routing_module_found {
            result
                .warnings
                .push("ルーティング設定が確認できません".to_string());
        }

        Ok(result)
    }

    // UI/UXデザインシステム解析
    pub fn analyze_ui_design_system(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "UI/UX・デザインシステム".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let scss_files = self.find_files_with_extension("scss")?;
        let css_files = self.find_files_with_extension("css")?;
        let mut design_tokens = false;
        let mut component_library = false;
        let mut theme_support = false;

        for file_path in scss_files.iter().chain(css_files.iter()) {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("$primary") || content.contains("--primary") {
                    design_tokens = true;
                }

                if content.contains("@mixin") || content.contains("@include") {
                    component_library = true;
                }

                if content.contains("theme") || content.contains("dark") {
                    theme_support = true;
                }
            }
        }

        result.details.push(format!(
            "デザイントークン: {}",
            if design_tokens {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "コンポーネントライブラリ: {}",
            if component_library {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "テーマサポート: {}",
            if theme_support {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        Ok(result)
    }

    // エラーハンドリング解析
    pub fn analyze_error_handling(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "エラーハンドリング・例外処理".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut global_error_handler = false;
        let mut try_catch_blocks = 0;
        let mut error_interceptor = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("ErrorHandler") {
                    global_error_handler = true;
                }

                if content.contains("HttpInterceptor") && content.contains("error") {
                    error_interceptor = true;
                }

                // try-catchブロックをカウント
                let try_regex = Regex::new(r"try\s*\{").unwrap();
                try_catch_blocks += try_regex.find_iter(&content).count();
            }
        }

        result.details.push(format!(
            "グローバルエラーハンドラー: {}",
            if global_error_handler {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result
            .details
            .push(format!("try-catchブロック数: {}", try_catch_blocks));
        result.details.push(format!(
            "エラーインターセプター: {}",
            if error_interceptor {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        if !global_error_handler {
            result
                .warnings
                .push("グローバルエラーハンドラーが確認できません".to_string());
        }

        Ok(result)
    }

    // パフォーマンス解析
    pub fn analyze_performance(&self) -> Result<AnalysisResult> {
        let mut result = AnalysisResult {
            category: "パフォーマンス".to_string(),
            status: "OK".to_string(),
            details: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        let ts_files = self.find_files_with_extension("ts")?;
        let mut lazy_loading = false;
        let mut change_detection = false;
        let mut virtual_scrolling = false;
        let mut service_worker = false;

        for file_path in &ts_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("loadChildren") {
                    lazy_loading = true;
                }

                if content.contains("OnPush") || content.contains("ChangeDetectionStrategy") {
                    change_detection = true;
                }

                if content.contains("cdk-virtual-scroll")
                    || content.contains("VirtualScrollStrategy")
                {
                    virtual_scrolling = true;
                }

                if content.contains("ServiceWorker") {
                    service_worker = true;
                }
            }
        }

        result.details.push(format!(
            "遅延読み込み: {}",
            if lazy_loading {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "変更検知最適化: {}",
            if change_detection {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "仮想スクロール: {}",
            if virtual_scrolling {
                "実装済み"
            } else {
                "未確認"
            }
        ));
        result.details.push(format!(
            "サービスワーカー: {}",
            if service_worker {
                "実装済み"
            } else {
                "未確認"
            }
        ));

        if !lazy_loading {
            result
                .warnings
                .push("遅延読み込みが確認できません".to_string());
        }

        Ok(result)
    }

    // ヘルパーメソッド
    fn find_files_with_extension(&self, extension: &str) -> Result<Vec<String>> {
        let mut files = Vec::new();
        self.find_files_recursive(&self.project_path, extension, &mut files)?;
        Ok(files)
    }

    fn find_files_recursive(
        &self,
        dir: &str,
        extension: &str,
        files: &mut Vec<String>,
    ) -> Result<()> {
        let path = Path::new(dir);
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().unwrap().to_str().unwrap();
                    if !dir_name.starts_with('.') && dir_name != "node_modules" {
                        self.find_files_recursive(path.to_str().unwrap(), extension, files)?;
                    }
                } else if let Some(file_extension) = path.extension() {
                    if file_extension == extension
                        || path.to_str().unwrap().ends_with(&format!(".{}", extension))
                    {
                        files.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn check_responsive_design(&self, css_files: &[String], scss_files: &[String]) -> Result<bool> {
        let media_query_regex = Regex::new(r"@media\s*\([^)]*\)").unwrap();

        for file_path in css_files.iter().chain(scss_files.iter()) {
            if let Ok(content) = fs::read_to_string(file_path) {
                if media_query_regex.is_match(&content) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

// メイン関数
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("使用方法: {} <プロジェクトパス>", args[0]);
        std::process::exit(1);
    }

    let project_path = args[1].clone();
    let analyzer = FrontendAnalyzer::new(project_path);

    println!("フロントエンド解析を開始します...\n");

    match analyzer.analyze_all() {
        Ok(results) => {
            for result in results {
                println!("=== {} ===", result.category);
                println!("ステータス: {}", result.status);

                if !result.details.is_empty() {
                    println!("詳細:");
                    for detail in &result.details {
                        println!("  ✓ {}", detail);
                    }
                }

                if !result.warnings.is_empty() {
                    println!("警告:");
                    for warning in &result.warnings {
                        println!("  ⚠ {}", warning);
                    }
                }

                if !result.errors.is_empty() {
                    println!("エラー:");
                    for error in &result.errors {
                        println!("  ✗ {}", error);
                    }
                }

                println!();
            }
        }
        Err(e) => {
            eprintln!("解析中にエラーが発生しました: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
