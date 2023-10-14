use crate::tantivy::{Document, Index, ReloadPolicy};
use tantivy;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, STORED, TEXT};

fn main() -> tantivy::Result<()> {
    let index_path = "./test";

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("node_id", STORED);
    schema_builder.add_text_field("node_text", TEXT);

    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let node_id = schema.get_field("node_id").unwrap();
    let node_text = schema.get_field("node_text").unwrap();

    if true {
        let mut index_writer = index.writer(50_000_000)?;

        {
            let mut document = Document::default();
            document.add_text(node_id, "5c268c6e-6888-4248-a517-abeea0f61f2b");
            document.add_text(
                node_text,
                "web application, that is written in TypeScript",
            );
            index_writer.add_document(document)?;
        }

        {
            let mut document = Document::default();
            document.add_text(node_id, "90e9a77b-274e-4db5-b029-aa76055e8aa2");
            document.add_text(
                node_text,
                "web application, that is written in JavaScript",
            );
            index_writer.add_document(document)?;
        }

        index_writer.commit()?;
    }

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![node_text]);

    {
        println!("Matches for \"application\":");
        let query = query_parser.parse_query("application")?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }
    }

    {
        println!("Matches for \"script\":");
        // TODO: adjust, so that matches are returned.
        let query = query_parser.parse_query("script")?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }
    }

    {
        println!("Matches for \"Type\":");
        // TODO: adjust, so that matches are returned.
        let query = query_parser.parse_query("Type")?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }
    }

    {
        println!("Matches for \"TypeScript\":");
        let query = query_parser.parse_query("TypeScript")?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }
    }

    println!("Done");

    Ok(())
}
