use std::fs::{create_dir_all, remove_dir};
use std::path::PathBuf;
use kuchiki::traits::TendrilSink;
use leptos::{view, CollectView, IntoView};
use thiserror::Error;
use crate::summarizer::DocumentationSection;

#[derive(Error, Debug)]
pub enum GenerationError {}

pub fn generate_documentation_site(directory: &PathBuf, summary: Vec<DocumentationSection>) -> Result<(), GenerationError> {
    let output_directory = directory.join("doc-site");
    remove_dir(output_directory.clone()).unwrap_or(());
    create_dir_all(output_directory.clone()).expect("Unable to create output directory");

    save_static_file(&output_directory, "style.css", include_bytes!("../design/style.css"));
    save_static_file(&output_directory, "script.js", include_bytes!("../design/script.js"));
    save_static_file(&output_directory, "Uiua386.ttf", include_bytes!("../design/Uiua386.ttf"));
    save_static_file(&output_directory, "index.html", generate_html(summary).as_bytes());

    // extract_doc_comments(&file.items).iter()
    //     .for_each(|comment| {
    //         println!("{}", comment);
    //     });

    Ok(())
}

fn save_static_file(output_directory: &PathBuf, file: &str, content: &[u8]) {
    let destination = output_directory.join(file);
    std::fs::write(destination, content).expect("Unable to write static file");
}

fn generate_html(summary: Vec<DocumentationSection>) -> String {
    let raw_output = leptos::ssr::render_to_string(|| generate_page(summary)).to_string();
    let document = kuchiki::parse_html().from_utf8().one(raw_output.as_bytes());

    // Remove comments
    document
        .inclusive_descendants()
        .filter(|node| node.as_comment().is_some())
        .for_each(|comment| {
            comment.detach()
        });

    // Remove data-hk attributes generated by leptos
    document
        .select("[data-hk]")
        .unwrap()
        .for_each(|node| {
            node.attributes.borrow_mut().remove("data-hk");
        });

    // Serialize back to string
    let mut result = Vec::new();
    document.serialize(&mut result).unwrap();
    String::from_utf8(result).unwrap()
}

fn generate_page(summary: Vec<DocumentationSection>) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>"Hello world"</title>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <link rel="stylesheet" href="style.css"/>
                <script src="script.js"></script>
            </head>
            <body>
                <div class="mobile-container">
                    <div class="mobile-nav">
                        <div class="hamburger">
                            <div class="line"></div>
                            <div class="line"></div>
                            <div class="line"></div>
                        </div>
                        <h1>"uiua-essentials"</h1>
                    </div>
                    <div class="container">
                        <div class="sidebar">
                            {generate_sidebar(&summary)}
                        </div>
                        <div class="content">
                            TODO
                        </div>
                    </div>
                </div>
            </body>
        </html>
    }
}

fn generate_sidebar(summary: &Vec<DocumentationSection>) -> impl IntoView {
    view! {
        {summary.iter()
            .map(|section| view! {
                <div class="sidebar-section">
                    <div class="section-name">{&section.title}</div>
                    <ul>
                        {section.content.iter()
                            .flat_map(|item| &item.links)
                            .map(|link| view! {
                                <li><a href={&link.url}>{&link.title}</a></li>
                            })
                            .collect_view()
                        }
                    </ul>
                </div>
            })
            .collect_view()
        }
    }
}