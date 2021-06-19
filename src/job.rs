pub fn return_color_from_job(job: &str) -> String {
    match job {
        "戦士" => "黄,黄".to_string(),
        "魔法使い" => "紫,紫".to_string(),
        "僧侶" => "緑,緑".to_string(),
        "武闘家"=> "赤,赤".to_string(),
        "盗賊"=> "青,青".to_string(),
        "遊び人"=> "青,紫".to_string(),
        "踊り子"=> "青,緑".to_string(),
        "レンジャー"=> "青青,青,青".to_string(),
        "賢者"=> "紫緑,紫緑,紫緑".to_string(),
        "バトルマスター"=> "赤黄,赤,赤".to_string(),
        "魔法戦士"=> "紫黄,紫黄,紫黄".to_string(),
        "パラディン"=> "黄緑,黄,黄".to_string(),
        "スパスタ"=> "青緑,青,緑".to_string(),
        "海賊"=> "黄青,黄,青".to_string(),
        _ => "".to_string()
    }
}
