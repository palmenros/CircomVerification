extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

mod compute_constants;
mod environment_utils;
mod execute;
mod execution_data;

use ansi_term::Colour;
use circom_algebra::algebra::{ArithmeticError, ArithmeticExpression};
use compiler::hir::very_concrete_program::VCP;
use constraint_writers::ConstraintExporter;
use dag::{DAG,TreeConstraints};
use execution_data::executed_program::ExportResult;
use execution_data::ExecutedProgram;
use program_structure::ast::{self};
use program_structure::constants::UsefulConstants;
use program_structure::error_code::ReportCode;
use program_structure::error_definition::{Report, ReportCollection};
use program_structure::file_definition::FileID;
use program_structure::program_archive::ProgramArchive;
use std::rc::Rc;

pub struct BuildConfig {
    pub flag_verbose: bool,
    pub inspect_constraints: bool,
}

pub type ConstraintWriter = Box<dyn ConstraintExporter>;
type BuildResponse = Result<(ConstraintWriter, VCP, TreeConstraints), ()>;
pub fn build_circuit(program: ProgramArchive, config: BuildConfig) -> BuildResponse {
    let files = program.file_library.clone();
    let exe = instantiation(&program, config.flag_verbose).map_err(|r| {
        Report::print_reports(&r, &files);
    })?;
    let (mut dag, mut vcp, warnings) = export(exe, program, config.flag_verbose).map_err(|r| {
        Report::print_reports(&r, &files);
    })?;
    if config.inspect_constraints {
        Report::print_reports(&warnings, &files);
    }
    
    let tree_constraints = dag.map_to_constraint_tree();
    sync_dag_and_vcp(&mut vcp, &mut dag);
    Result::Ok((Box::new(dag), vcp, tree_constraints))

}

type InstantiationResponse = Result<ExecutedProgram, ReportCollection>;
fn instantiation(program: &ProgramArchive, flag_verbose: bool) -> InstantiationResponse {
    let execution_result = execute::constraint_execution(&program, flag_verbose);
    match execution_result {
        Ok(program_exe) => {
            let no_nodes = program_exe.number_of_nodes();
            let success = Colour::Green.paint("template instances");
            let nodes_created = format!("{}: {}", success, no_nodes);
            println!("{}", &nodes_created);
            InstantiationResponse::Ok(program_exe)
        }
        Err(reports) => InstantiationResponse::Err(reports),
    }
}

fn export(exe: ExecutedProgram, program: ProgramArchive, flag_verbose: bool) -> ExportResult {
    let exported = exe.export(program, flag_verbose);
    exported
}

fn sync_dag_and_vcp(vcp: &mut VCP, dag: &mut DAG) {
    let witness = Rc::new(DAG::produce_witness(dag));
    VCP::add_witness_list(vcp, Rc::clone(&witness));
}
