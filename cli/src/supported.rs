const SUPPORTED_ENCODERS: [&'static str; 4] = ["h264_nvenc", "hevc_nvenc", "h264_amf", "hevc_amf"];
const DOWNLOAD_URL: &str = "https://www.dropbox.com/sh/x08pkk47lc1v5ex/AADGaoOjOcA0-uPo7I0NaxL-a?dl=0";
const ENCODE_FILES: [&'static str; 8] = ["720-60.y4m", "720-120.y4m", "1080-60.y4m", "1080-120.y4m", "2k-60.y4m", "2k-120.y4m", "4k-60.y4m", "4k-120.y4m"];

pub fn is_encoder_supported(potential_encoder: &String) -> bool {
    return SUPPORTED_ENCODERS.contains(&potential_encoder.as_str());
}

pub fn get_supported_encoders() -> [&'static str; 4] {
    return SUPPORTED_ENCODERS;
}

pub fn get_download_url() -> &'static str {
    return DOWNLOAD_URL;
}

pub fn get_supported_inputs() -> [&'static str; 8] {
    return ENCODE_FILES;
}