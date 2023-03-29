use flate2::Decompress;
pub parse_file(file_path: String){
    data_file = DataFile(file_path);

    if is_gzip(data_file) {
        handle = open_gzip(file_path)
    }else{
        handle = open(file_path)
    }
    for line in handle:
        temp = line.strip().lower();
        if length(temp) == 0 or temp.startswith("#"):
            continue;
        else {
            data_file.add_data(line);
        }
    return data_file
}

fn open_gzip(path: String) -> Vec<String>{
    
