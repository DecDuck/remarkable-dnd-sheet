#!/usr/bin/env python3
"""
Render page 1 of a PDF character sheet to a PNG and extract all AcroForm
field bounding-boxes so the reMarkable app can overlay ink / interactive
widgets at exactly the right positions.

Usage
-----
    python3 tools/extract_fields.py path/to/character_sheet.pdf

Outputs
-------
  assets/charsheet_bg.png          – background image (copy to /home/root/ on device)
  assets/charsheet_fields.json     – field map (copy to /home/root/ on device)

The app loads both files at runtime from /home/root/.  Re-run this script
whenever you use a different sheet PDF; then scp both files to the device.

Dependencies
------------
    pip install pymupdf          # provides the `fitz` package (PyMuPDF)

Field JSON schema
-----------------
Each entry in the top-level array:
  {
    "id":    <AcroForm field name, e.g. "CharacterName">,
    "label": <human-readable label, may be same as id>,
    "type":  "text" | "checkbox",
    "x":     <left edge,   normalised 0–1 relative to page width>,
    "y":     <top edge,    normalised 0–1 relative to page height>,
    "w":     <field width, normalised>,
    "h":     <field height, normalised>
  }

Normalised coordinates map to screen pixels as:
    screen_x = image_origin_x + field.x * image_painted_width
    screen_y = image_origin_y + field.y * image_painted_height
"""

import argparse
import json
import sys
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Render a PDF sheet and extract its AcroForm field positions."
    )
    parser.add_argument("pdf", help="Path to the character sheet PDF")
    parser.add_argument(
        "--dpi",
        type=int,
        default=200,
        help="Render resolution (default: 200 dpi → ~1700×2200 px for Letter)",
    )
    parser.add_argument(
        "--page",
        type=int,
        default=0,
        help="Zero-based page index to render (default: 0)",
    )
    parser.add_argument(
        "--out-dir",
        default=None,
        help="Output directory (default: <repo_root>/assets/)",
    )
    args = parser.parse_args()

    try:
        import fitz  # PyMuPDF
    except ImportError:
        sys.exit(
            "PyMuPDF not found. Install it with:\n"
            "    pip install pymupdf\n"
        )

    pdf_path = Path(args.pdf)
    if not pdf_path.exists():
        sys.exit(f"PDF not found: {pdf_path}")

    out_dir = Path(args.out_dir) if args.out_dir else Path(__file__).parent.parent / "assets"
    out_dir.mkdir(parents=True, exist_ok=True)

    doc = fitz.open(str(pdf_path))
    if args.page >= len(doc):
        sys.exit(f"PDF has {len(doc)} page(s); page index {args.page} is out of range.")

    page = doc[args.page]
    pw, ph = page.rect.width, page.rect.height  # PDF points (72 pt = 1 inch)

    # ── Render to PNG ──────────────────────────────────────────────────────
    scale = args.dpi / 72.0
    mat = fitz.Matrix(scale, scale)
    pix = page.get_pixmap(matrix=mat, alpha=False)

    png_path = out_dir / "charsheet_bg.png"
    pix.save(str(png_path))
    print(f"Background: {png_path}  ({pix.width}×{pix.height} px at {args.dpi} dpi)")

    # ── Extract AcroForm widget positions ──────────────────────────────────
    fields = []
    seen_ids: set[str] = set()

    for widget in page.widgets():
        ftype = widget.field_type_string
        if ftype not in ("Text", "CheckBox", "RadioButton"):
            continue

        # Unique-ify duplicate names (e.g. radio group members)
        base_id = widget.field_name or f"field_{len(fields)}"
        uid = base_id
        suffix = 1
        while uid in seen_ids:
            uid = f"{base_id}_{suffix}"
            suffix += 1
        seen_ids.add(uid)

        r = widget.rect
        fields.append(
            {
                "id": uid,
                "label": (widget.field_label or base_id).strip() or uid,
                "type": "checkbox" if ftype in ("CheckBox", "RadioButton") else "text",
                # Normalised to [0, 1] relative to the page dimensions
                "x": round(r.x0 / pw, 6),
                "y": round(r.y0 / ph, 6),
                "w": round(r.width / pw, 6),
                "h": round(r.height / ph, 6),
            }
        )

    json_path = out_dir / "charsheet_fields.json"
    json_path.write_text(json.dumps(fields, indent=2))
    print(f"Field map:  {json_path}  ({len(fields)} fields)")

    print()
    print("Deploy to device:")
    print(f"  scp {png_path}  root@10.61.28.49:/home/root/")
    print(f"  scp {json_path} root@10.61.28.49:/home/root/")


if __name__ == "__main__":
    main()
