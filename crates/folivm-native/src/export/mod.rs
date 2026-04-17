// Export pipeline for PDF/UA and DOCX.
//
// Both formats use folivm-core's LayoutEngine (cosmic-text) loaded with the
// same font bytes as the WASM editor. This guarantees numerically identical
// line breaks and page breaks between the editor and the exported file.
//
// Atomic write protocol: output is written to a temp file adjacent to the
// destination and renamed on completion. A failed export must not corrupt
// the previous output file.
