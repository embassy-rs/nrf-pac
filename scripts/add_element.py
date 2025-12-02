#!/usr/bin/env python3
"""
Generic script to add XML elements to nodes matching an XPath query.
Uses lxml for XML processing. Preserves comments and structure.
Note: Namespace attributes may be reordered per XML spec.
"""

import argparse
import shutil
import sys
from pathlib import Path
from copy import deepcopy

try:
    from lxml import etree
    HAS_LXML = True
except ImportError:
    HAS_LXML = False


def validate_xml_snippet(snippet: str) -> etree._Element:
    """Validate and parse an XML snippet."""
    try:
        return etree.fromstring(snippet.encode())
    except etree.XMLSyntaxError as e:
        raise ValueError(f"Invalid XML snippet: {e}")


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
    if elem.get('id'):
        match_id += f"[@id='{elem.get('id')}']"
    else:
        name_elem = elem.find('name')
        if name_elem is not None and name_elem.text:
            match_id += f"[name='{name_elem.text}']"
    return match_id


def add_element_to_matches(xml_file: str, xpath: str, snippet: str) -> int:
    """Add an XML element to all nodes matching the XPath query."""
    # Validate the snippet
    snippet_element = validate_xml_snippet(snippet)
    
    # Parse preserving comments, but allow reformatting for consistent output
    parser = etree.XMLParser(remove_blank_text=True, remove_comments=False)
    tree = etree.parse(xml_file, parser)
    root = tree.getroot()
    
    # Find matches
    matches = validate_xpath(tree, xpath)
    
    if not matches:
        print(f"  No elements matched XPath: {xpath}")
        return 0
    
    modified_count = 0
    
    for match in matches:
        # Deep copy the snippet for each insertion
        new_element = deepcopy(snippet_element)
        match.append(new_element)
        modified_count += 1
        print(f"  Added element to: {get_element_identifier(match)}")
    
    if modified_count > 0:
        tree.write(xml_file, encoding='UTF-8', xml_declaration=True, pretty_print=True)
    
    return modified_count


def main():
    if not HAS_LXML:
        print("Error: This script requires lxml. Install it with: pip install lxml", 
              file=sys.stderr)
        sys.exit(1)
    
    parser = argparse.ArgumentParser(
        description='Add XML elements to nodes matching an XPath query',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s file.xml --xpath ".//peripheral[name='CLOCK']" --snippet "<interrupt><name>IRQ</name></interrupt>"
  %(prog)s file.xml --xpath ".//device" --snippet "<version>1.0</version>"
  %(prog)s file.xml --xpath ".//config" --snippet-file element.xml
  %(prog)s file.xml --xpath ".//peripheral" --snippet "<enabled/>" --dry-run
        """
    )
    parser.add_argument('xml_file', help='Path to the XML file to modify')
    parser.add_argument('--xpath', '-x', required=True,
                        help='XPath query to find target elements')
    parser.add_argument('--snippet', 
                        help='XML snippet to add (as string)')
    parser.add_argument('--snippet-file', '-s',
                        help='Read XML snippet from file instead of --snippet')
    parser.add_argument('--dry-run', action='store_true',
                        help='Show what would be modified without making changes')
    
    args = parser.parse_args()
    
    xml_path = Path(args.xml_file)
    
    if not xml_path.exists():
        print(f"Error: File '{args.xml_file}' not found", file=sys.stderr)
        sys.exit(1)
    
    # Get XML snippet
    if args.snippet_file:
        snippet_path = Path(args.snippet_file)
        if not snippet_path.exists():
            print(f"Error: Snippet file '{args.snippet_file}' not found", file=sys.stderr)
            sys.exit(1)
        snippet = snippet_path.read_text().strip()
    elif args.snippet:
        snippet = args.snippet
    else:
        print("Error: XML snippet required (use --snippet or --snippet-file)", 
              file=sys.stderr)
        sys.exit(1)
    
    # Validate inputs
    print("Validating inputs...")
    
    try:
        validate_xml_snippet(snippet)
        print("  XML snippet: valid")
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    
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
    
    if args.dry_run:
        print(f"\nDry run - would modify {len(matches)} element(s)")
        for match in matches:
            print(f"  Would add to: {get_element_identifier(match)}")
        sys.exit(0)
    
    # Process the file
    print(f"\nProcessing: {args.xml_file}")
    
    try:
        modified = add_element_to_matches(args.xml_file, args.xpath, snippet)
        
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
