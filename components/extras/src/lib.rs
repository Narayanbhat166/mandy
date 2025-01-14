/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// that creates files for
/// Deno Deploy.
pub use modules::deno::*;

/// Re-exporting the module that
/// compiles SCSS files.
pub use modules::sass::*;

/// Re-exporting the module
/// that generates 
/// a "robots.txt" file.
pub use modules::robots::*;

/// Re-exporting the module
/// that creates files for
/// SEO crawlers.
pub use modules::crawlers::*;

/// Re-exporting the module
/// that generates 
/// a "robots.txt" file.
pub use modules::site_map::*;

/// Re-exporting the module
/// that generates 
/// a build info file.
pub use modules::build_meta::*;