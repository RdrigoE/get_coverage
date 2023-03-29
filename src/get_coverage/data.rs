use std::collections::HashMap;
struct depth_entry(i32, i32, i32);

pub struct DataFile {
    file_name: String,
    vect_chromosomes: Vec<depth_entry>,
    dict_data: HashMap<i32, depth_entry>,
    dict_data_coverage: HashMap,
    previous_position: u32,
}

impl DataFile {
    fn add_data(&mut self, line: String) -> Option<(), None>{
        if len(line) == 0 || line[0] == '#' {
            return None;
        }
        let vect_data: Vec<String> = line.split("\t").collect();
        if len(vect_data) != 3 {
            panic!("Input file must have rows with 3 values");
        }
        entry = depth_entry(vect_data[0], vect_data[1], vect_data[2]);
        if !(self.dict_data.contains_key(entry.0)){
            self.vect_chromosomes.push(entry.0);
            self.dict_data.insert(entry.0, (entry.1, entry.2));
            self.previous_position = entry.1;
            return Some(_);
        }
        else{
            if entry.1 <= self.previous_position {
                raise panic!(format!("File: {} | Line: {} | The locus need to be greater than the predecessor in the file", self.file_name , line ));
            }
            self.dict_data[entry.0].append((entry.1, entry.2));
            self.previous_position = entry.1;
            return Some(_);
        }
    }
}

fn get_coverage(&mut self, sz_chromosome: i32, length_chromosome: f64){
    if !self.dict_data.contains_key(sz_chromosome) {
        return 0;
    }
    if self.dict_data_coverage.has_key(sz_chromosome){
        return self.dict_data_coverage[sz_chromosome]
    }
    if (length_chromosome == 0) {
        return 0;}
    if (len(self.dict_data[sz_chromosome]) > length_chromosome){
         panic!(format!("Chromosome {} has different sizes. Coverage: {}; Reference: {}", (sz_chromosome, len(self.dict_data[sz_chromosome]), length_chromosome)));
    }
    let sum_total: f64 = 0;
    for data_ in self.dict_data[sz_chromosome]{
        sum_total += data_[1]
    }
    self.dict_data_coverage[sz_chromosome] = sum_total / length_chromosome;
    return self.dict_data_coverage[sz_chromosome];
}
fn get_ratio_more_than(&self, sz_chromosome: i32, length_chromosome: i32, value: i32){
    if (!self.dict_data.has_key(sz_chromosome)) {
        return 0;
    }
    if (length_chromosome == 0) {
        return 0;
    }
    if (len(self.dict_data[sz_chromosome]) > length_chromosome){
        panic!(format!("Chromosome {} has different sizes. Coverage: {}; Reference: {}" , (sz_chromosome, len(self.dict_data[sz_chromosome]), length_chromosome)));
    }
    let sum_total = 0;
    for data_ in self.dict_data[sz_chromosome]{
        if data_[1] > value{
            sum_total +=1
        }
    }
    return sum_total / float(length_chromosome);
}
fn get_file_name(&self){
    let sz_return = os.path.basename(self.file_name);
    if sz_return.rfind(".gz") == len(sz_return) - 3 {
        sz_return = sz_return[..3];
    }
    if sz_return.rfind(".") != -1{
        sz_return = sz_return[..1 * (len(sz_return) - sz_return.rfind("."))];
    }
    return sz_return;
}
