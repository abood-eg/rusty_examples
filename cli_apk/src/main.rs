struct Hell{
    fire:i32,
    s3eer:f32
}

impl Hell{
    fn print_hell(&self){
        print!("{}",self.fire);
    }
    fn print_sa3eer(&self){
        print!("{}",self.s3eer);
    }
}



fn main() {
    let fi=Hell{fire:20,s3eer:32.2};
    fi.print_hell();
    fi.print_sa3eer();
}
