#!/usr/bin/env python3
"""
Generic script to set the text of XML elements matching an XPath query.
Uses lxml for XML processing. Preserves comments and structure.
Note: Namespace attributes may be reordered per XML spec.
"""

import argparse
import sys
from pathlib import Path

try:
    from lxml import etree
    HAS_LXML = True
except ImportError:
    HAS_LXML = False


def validate_xpath(tree: etree._ElementTree, xpath: str) -> list:
    """Validate an XPath query and return matching elements."""
    try:
        matches = tree.xpath(xpath)
        return [m for m in matches if isinstance(m, etree._Element)]
    except etree.XPathError as e:
        raise ValueError(f"Invalid XPath expression: {e}")


def get_element_identifier(elem) -> str:
    """Get a human-readable identifier for an element."""
    match_id = etree.QName(elem.tag).localname if '}' in elem.tag else elem.tag
    parent = elem.getparent()
    if parent is not None:
        name_elem = parent.find('name')
        if name_elem is not None and name_elem.text:
            match_id = f"{parent.tag}[name='{name_elem.text}']/{match_id}"
    return match_id


def modify_text_of_matches(xml_file: str, xpath: str, value: str) -> int:
    """Set the text of all elements matching the XPath query."""
    # Parse preserving comments, but allow reformatting for consistent output
    parser = etree.XMLParser(remove_blank_text=True, remove_comments=False)
    tree = etree.parse(xml_file, parser)

    # Find matches
    matches = validate_xpath(tree, xpath)

    if not matches:
        print(f"  No elements matched XPath: {xpath}")
        return 0

    modified_count = 0

    for match in matches:
        # Skip elements that already have the value (idempotency).
        if match.text == value:
            print(f"  Already set in: {get_element_identifier(match)}, skipping")
            continue

        print(f"  Changed {get_element_identifier(match)}: {match.text} -> {value}")
        match.text = value
        modified_count += 1

    if modified_count > 0:
        tree.write(xml_file, encoding='UTF-8', xml_declaration=True, pretty_print=True)

    return modified_count


def main():
    if not HAS_LXML:
        print("Error: This script requires lxml. Install it with: pip install lxml",
              file=sys.stderr)
        sys.exit(1)

    parser = argparse.ArgumentParser(
        description='Set the text of XML elements matching an XPath query',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s file.xml --xpath ".//register[name='CTRL']/size" --value "0x20"
  %(prog)s file.xml --xpath ".//register[name='TASKS[%%s]']/dim" --value "0x17" --dry-run
        """
    )
    parser.add_argument('xml_file', help='Path to the XML file to modify')
    parser.add_argument('--xpath', '-x', required=True,
                        help='XPath query to find target elements')
    parser.add_argument('--value', '-v', required=True,
                        help='New text of the target elements')
    parser.add_argument('--dry-run', action='store_true',
                        help='Show what would be modified without making changes')

    args = parser.parse_args()

    xml_path = Path(args.xml_file)

    if not xml_path.exists():
        print(f"Error: File '{args.xml_file}' not found", file=sys.stderr)
        sys.exit(1)

    # Validate inputs
    print("Validating inputs...")

    try:
        parser_obj = etree.XMLParser(remove_blank_text=False, remove_comments=False)
        tree = etree.parse(args.xml_file, parser_obj)
        matches = validate_xpath(tree, args.xpath)
        print(f"  XPath query: valid ({len(matches)} match(es) found)")
    except etree.XMLSyntaxError as e:
        print(f"Error: Failed to parse XML file: {e}", file=sys.stderr)
        sys.exit(1)
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    if not matches:
        print(f"Error: No elements matched XPath: {args.xpath}", file=sys.stderr)
        sys.exit(1)

    if args.dry_run:
        print(f"\nDry run - would modify {len(matches)} element(s)")
        for match in matches:
            print(f"  Would set {get_element_identifier(match)}: {match.text} -> {args.value}")
        sys.exit(0)

    # Process the file
    print(f"\nProcessing: {args.xml_file}")

    try:
        modified = modify_text_of_matches(args.xml_file, args.xpath, args.value)

        if modified > 0:
            print(f"\nSuccessfully modified {modified} element(s)")
        else:
            print("\nNo elements were modified")

    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
