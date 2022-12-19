use std::{io::{BufWriter}, fs::File, path::PathBuf, fmt::format};

use args::{Args, PlotMode};
use clap::Parser;
use spwm_generator::*;
use plotters::prelude::*;
use rayon::prelude::*;

mod args;
mod format;

use format::*;

const INVERTER_OFFSET: f32 = 1.5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let duration = if args.one_cycle {
        if args.sin_freq > 0.0 {
            1.0 / args.sin_freq
        }
        else {
            0.0
        }
    }
    else {
        args.duration
    };


    let (num_of_samples, step) = SPWM::convert(
        duration, 
        args.num_of_samples, 
        args.step, 
        args.carrier_freq
    )?;

    let spwm = SPWM::new(
        args.sin_freq, 
        num_of_samples, 
        step,
        args.pwm_top, 
        args.padding
    );
    
    let mut fs = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&args.output)?;
    let writer: &dyn Format = match args.format {
        args::Format::Raw => &Raw,
        args::Format::RawHex => &RawHex,
        args::Format::C => &CFile,
        args::Format::CHex => &CHexFile,
        args::Format::Rust => &RustFile,
        args::Format::RustHex => &RustHexFile,
    };
    let format_args = FormatArgs {
        name: args.name,
        separator: args.separator,
        width: args.row_width,
        inverter: args.inverter,
    };
    writer.write(&spwm, &mut fs, &format_args)?;

    if let Some(mode) = args.plot {
        plot_wave(mode, &spwm, &args.output, &format_args)?;
    }

    Ok(())
}

fn plot_wave(mode: PlotMode, spwm: &SPWM, path: &PathBuf, args: &FormatArgs) -> Result<(), Box<dyn std::error::Error>> {
    //let path = path.ancestors().nth(1).unwrap().join(format!("Wave_{}.svg", spwm.sin_freq()));
    let mut path = path.clone();
    path.set_extension("svg");

    let table = spwm.lookup_table();
    let len = table.len() as f32;

    let root = SVGBackend::new(&path, (len as u32 * 100 + 200, 480)).into_drawing_area();
    let caption = format!("SPWM Wave {} Hz", spwm.sin_freq());

    let min_val = if let Some(_) = args.inverter {
        -INVERTER_OFFSET
    }
    else {
        0.0
    };

    // generate samples
    let samples: Vec<(f32, f32)> = table.par_iter().enumerate().flat_map(|(idx, v)| {
        if *v != 0 {
            let w = *v as f32 / spwm.pwm_top() as f32;
            match mode {
                PlotMode::PWM => {
                    vec![
                        (idx as f32, 1f32),
                        (idx as f32 + w, 1f32),
                        (idx as f32 + w, 0f32),
                        (idx as f32 + 1f32, 0f32),
                    ]
                },
                PlotMode::CenterAligned => {
                    let pad = (1f32 - w) / 2f32;
                    vec![
                        (idx as f32, 0f32),
                        (idx as f32 + pad, 0f32),
                        (idx as f32 + pad, 1f32),
                        (idx as f32 + pad + w, 1f32),
                        (idx as f32 + pad + w, 0f32),
                        (idx as f32 + 1f32, 0f32),
                    ]
                },
            }
        }
        else if *v == spwm.pwm_top() {
            vec![
                (idx as f32, 1f32),
                (idx as f32 + 1f32, 1f32),
            ]
        }
        else {
            vec![
                (idx as f32, 0f32),
                (idx as f32 + 1f32, 0f32),
            ]
        }
    }).collect();

    let samples_inverted: Option<Vec<(f32, f32)>> = if let Some(pad) = args.inverter {
        let table_not = spwm.table_not(&table, pad);
        Some(table_not.par_iter().enumerate().flat_map(|(idx, v)| {
            if *v != 0 {
                let w = *v as f32 / spwm.pwm_top() as f32;
                match mode {
                    PlotMode::PWM => {
                        vec![
                            (idx as f32, 1f32 - INVERTER_OFFSET),
                            (idx as f32 + w, 1f32 - INVERTER_OFFSET),
                            (idx as f32 + w, 0f32 - INVERTER_OFFSET),
                            (idx as f32 + 1f32, 0f32 - INVERTER_OFFSET),
                        ]
                    },
                    PlotMode::CenterAligned => {
                        let pad = (1f32 - w) / 2f32;
                        vec![
                            (idx as f32, 0f32 - INVERTER_OFFSET),
                            (idx as f32 + pad, 0f32 - INVERTER_OFFSET),
                            (idx as f32 + pad, 1f32 - INVERTER_OFFSET),
                            (idx as f32 + pad + w, 1f32 - INVERTER_OFFSET),
                            (idx as f32 + pad + w, 0f32 - INVERTER_OFFSET),
                            (idx as f32 + 1f32, 0f32 - INVERTER_OFFSET),
                        ]
                    },
                }
            }
            else if *v == spwm.pwm_top() {
                vec![
                    (idx as f32, 1f32 - INVERTER_OFFSET),
                    (idx as f32 + 1f32, 1f32 - INVERTER_OFFSET),
                ]
            }
            else {
                vec![
                    (idx as f32, 0f32 - INVERTER_OFFSET),
                    (idx as f32 + 1f32, 0f32 - INVERTER_OFFSET),
                ]
            }
        }).collect())
    }
    else {
        None
    };

    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..len, min_val..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            samples,
            &RED,
        ))?;

    if let Some(inverted) = samples_inverted {
        chart
        .draw_series(LineSeries::new(
            inverted,
            &BLUE,
        ))?;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
