mod lyrics;

use reqwest::Client;
use crate::lyrics::chat_qwen::ChatQwen;
use crate::lyrics::chat_deepseek::ChatDs;
use crate::lyrics::lyrics::Lyrics;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_key = "V9tfY7wrZmiTV019qGYSc8Q6b8nKPlqK6sglYPxowJpBrpQ91UM196pzNE45Qx3r"; // 替换为你的 API 密钥
    let song_title = "Lady Writer"; // 替换为你想查询的歌曲名称
    let artist_name = ""; // 替换为歌手名称
    let qwen_api_key = "sk-510ed600fa2342ffbb88d53931bb70b0";
    let ds_api_key = "sk-da4796912b01421d9be824b50473ab98";

    // 获取歌词
    let lyrics = Lyrics::fetch_lyrics(&client, api_key, song_title, artist_name).await?;

    // 对照翻译为中文
    let translated_lyrics = ChatDs::exec_translate(&lyrics, ds_api_key).await.unwrap();
    println!("Translated Lyrics:\n{}", translated_lyrics);

    Ok(())
}
