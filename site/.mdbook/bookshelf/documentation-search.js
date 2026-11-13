(() => {
const metadata = readDocumentationPageMetadata();
if (typeof metadata.searchIndexTarget === "string" && metadata.searchIndexTarget !== "") {
window.path_to_searchindex_js = metadata.searchIndexTarget;
}

function readDocumentationPageMetadata() {
const node = document.getElementById("mdbook-bookshelf-page-metadata");
if (!node) {
return {};
}
try {
return JSON.parse(node.textContent || "{}");
} catch (_err) {
return {};
}
}
})();
