(() => {
const documentationIndexLabel = "Meus Estudos";
const metadata = readDocumentationPageMetadata();
const documentationIndexTarget = typeof metadata.documentationIndexTarget === "string" ? metadata.documentationIndexTarget : "";
if (!documentationIndexTarget) {
    return;
}
const buttonContainer = document.querySelector("#mdbook-menu-bar .right-buttons");
if (!buttonContainer || document.getElementById("documentation-return-link")) {
    return;
}
const svgNamespace = "http://www.w3.org/2000/svg";
const link = document.createElement("a");
link.id = "documentation-return-link";
link.className = "documentation-return-link";
link.href = documentationIndexTarget;
link.rel = "up";
link.title = "Return to " + documentationIndexLabel;
link.setAttribute("aria-label", "Return to " + documentationIndexLabel);

const icon = document.createElementNS(svgNamespace, "svg");
icon.setAttribute("class", "documentation-return-icon");
icon.setAttribute("viewBox", "0 0 24 24");
icon.setAttribute("fill", "none");
icon.setAttribute("stroke", "currentColor");
icon.setAttribute("stroke-width", "2");
icon.setAttribute("stroke-linecap", "round");
icon.setAttribute("stroke-linejoin", "round");
icon.setAttribute("aria-hidden", "true");
icon.setAttribute("focusable", "false");

const shapes = [
    ["rect", { x: "3", y: "4", width: "3.5", height: "14", rx: "0.5" }],
    ["rect", { x: "8.5", y: "4", width: "3.5", height: "14", rx: "0.5" }],
    ["rect", { x: "14", y: "4", width: "3.5", height: "14", rx: "0.5" }],
    ["line", { x1: "2", y1: "20", x2: "22", y2: "20" }],
];
for (const [tag, attrs] of shapes) {
    const node = document.createElementNS(svgNamespace, tag);
    for (const name in attrs) {
        node.setAttribute(name, attrs[name]);
    }
    icon.appendChild(node);
}

const label = document.createElement("span");
label.className = "documentation-return-label";
label.textContent = documentationIndexLabel;

link.appendChild(icon);
link.appendChild(label);
buttonContainer.prepend(link);

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
