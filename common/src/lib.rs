pub mod cli {
    pub mod action;
    pub mod context;
    pub mod spinner;

    pub mod print {
        mod child_output;
        mod md;
        mod tldr;
        mod version;

        pub use {
            child_output::print_child_output as child_output, md::print_md as md,
            tldr::print_tldr as tldr, version::print_version as version,
        };
    }
}

pub mod git;
