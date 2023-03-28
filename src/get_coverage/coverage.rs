use std::io::ErrorKind;
use std::path::PathBuf;
pub fn process_files(input: String, reference: String, output: String, ratio: String) {
    let input_path = get_input_file(input).ok();

    let reference_path = get_input_file(reference).ok();


    // Initiate ParseFile


}

pub fn get_input_file(input: String) -> Result<String, ErrorKind> {
    let path = PathBuf::from(&input);
    if path.is_file(){
        return Ok(input)
    }
    else {
        return Err(ErrorKind::NotFound)
    }
}
/*
def process_files(self, input_file, reference, output_file, ratio):

        ## read config file
        self.test_input_files(input_file)

        ## test fasta reference
        self.read_reference_fasta(reference)

        parse_file = ParseFile()
        vect_data = []
        for file_to_process in self.vect_files_processed:
            print "processing file: " + file_to_process
            if (not os.path.exists(file_to_process)):
                print "File doesn't exist: " + file_to_process
                continue
            vect_data.append(parse_file.parse_file(file_to_process))

        ###
        if (len(vect_data) == 0): sys.exit("There's no data to process")
        if (ratio is None): ratio = '0,9'
        vect_ratios = []
        for i in ratio.split(','):
            if (not self.utils.is_integer(i)): continue
            vect_ratios.append(i)

        handle = open(output_file, "w")
        handle.write("\nChromosome\n" + self.__get_chromosome__() + "Length")
        for chromosome in self.vect_reference:
            if (not self.reference_dict.has_key(chromosome)): raise Exception("Can't locate the chromosome '" + chromosome + "' in reference file")
            handle.write("\t%d" % (self.reference_dict[chromosome]))
        handle.write("\n")

        handle.write("\nCoverage\t" + "\t" * len(self.vect_reference))
        for ratio in vect_ratios:
            handle.write("\tRatio>%s" % (ratio)	+ "\t" * len(self.vect_reference))
        handle.write("\n")
        for data_from_file in vect_data:
            handle.write(data_from_file.get_file_name())
            sz_out = ""
            for chromosome in self.vect_reference:
                if (not self.reference_dict.has_key(chromosome)): raise Exception("Can't locate the chromosome '" + chromosome + "' in reference file")
                sz_out += "\t%.2f" % (data_from_file.get_coverage(chromosome, self.reference_dict[chromosome]))
            sz_out += "\t"

            for i in vect_ratios:
                for chromosome in self.vect_reference:
                    sz_out += "\t%.1f" % (data_from_file.get_ratio_more_than(chromosome, self.reference_dict[chromosome], int(i)) * 100)
                if i != vect_ratios[-1]: sz_out += "\t"
            handle.write(sz_out + "\n")

        handle.close()
        print "Output saved in: " + output_file
        print "Finished..."
*/
