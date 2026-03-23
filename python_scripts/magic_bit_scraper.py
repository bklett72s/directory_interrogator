import xml.etree.ElementTree as ET
from pathlib import Path

def extract_magic_bit_data():
    script_dir = Path(__file__).parent.parent

    file_path = script_dir / 'data' / 'DROID_SignatureFile_V122.xml'

    tree = ET.parse(file_path)
    root = tree.getroot()

    # namespace definition
    ns = {"pronom": "http://www.nationalarchives.gov.uk/pronom/SignatureFile"}

    # list for storing extracted data
    xml_InternalSignature_data = []
    xml_FileFormat_data = []
    xml_Matched_data = []

    for sig in root.findall(".//pronom:InternalSignature", ns):
        sig_id = sig.get("ID")

        sequence = sig.find(".//pronom:Sequence", ns)
        default_shift = sig.find(".//pronom:DefaultShift", ns)

        seq_val = sequence.text if sequence is not None else None
        shift_val = default_shift.text if default_shift is not None else None

        #print({
        #    "id": sig_id,
        #    "sequence": seq_val,
        #    "default_shift": shift_val
        #})

        xml_InternalSignature_data.append({
            "id": sig_id,
            "sequence": seq_val,
            "default_shift": shift_val
        })

    for fmt in root.findall(".//pronom:FileFormat", ns):
        fmt_id = ""
        for sig_ref in fmt.findall(".//pronom:InternalSignatureID", ns):
            fmt_id = sig_ref.text
            break
        fmt_name = fmt.get("Name")
        fmt_version = fmt.get("Version")

        fmt_extension = fmt.find(".//pronom:Extension", ns)
        fmt_extension_val = fmt_extension.text if fmt_extension is not None else None 

        #print({
        #    "id": fmt_id,
        #    "name": fmt_name,
        #    "version": fmt_version,
        #    "extension": fmt_extension_val
        #})

        xml_FileFormat_data.append({
            "id": fmt_id,
            "name": fmt_name,
            "version": fmt_version,
            "extension": fmt_extension_val
        })

    # Compare InternalSignauture and FileFormat data
    for sig in xml_InternalSignature_data:
        for fmt in xml_FileFormat_data:
            if sig["id"] == fmt["id"]:
                #print(f"Match found for ID {sig['id']}:")
                #print(f"  InternalSignature: {sig}")
                #print(f"  FileFormat: {fmt}")
                xml_Matched_data.append({
                    "Name": fmt["name"],
                    "Signature": sig["sequence"],
                    "extension": fmt["extension"],
                })

    xml_Matched_data.sort(key=lambda x: x["Name"])

    print(xml_Matched_data)

    return xml_Matched_data