use anyhow::Result;
use nimbus_experiments::{AppContext, Experiments};
fn main() -> Result<()> {
    viaduct_reqwest::use_reqwest_backend();
    let exp = Experiments::new(AppContext::default(), "./mydb", None);
    let enrolled_exp = exp.get_enrolled_experiments();
    exp.get_experiments().iter().for_each(|e| {
        print!(
            "Experiment: \"{}\", Buckets: {} to {}, Branches: ",
            e.id, e.arguments.bucket_config.count, e.arguments.bucket_config.start
        );
        e.arguments
            .branches
            .iter()
            .for_each(|b| print!(" \"{}\", ", b.slug));
        println!()
    });
    println!("You are in bucket: {}", exp.get_bucket());
    enrolled_exp.iter().for_each(|ee| {
        println!(
            "Enrolled in experiment \"{}\" in branch \"{}\"",
            ee.get_id(),
            ee.get_branch()
        )
    });
    Ok(())
}
