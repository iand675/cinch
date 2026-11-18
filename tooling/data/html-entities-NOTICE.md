# HTML named character reference data

`html-entities.json` freezes the 2,231-entry `html.entities.html5` mapping
distributed with Python 3.12.14. The mapping represents the HTML named character
references, including the 106 legacy names without semicolons and replacements
containing two Unicode scalars. Python's source table cites the WHATWG HTML
named character reference data.

The Python distribution license is retained verbatim in
`html-entities-PYTHON-LICENSE.txt`. No executable Python library source is
vendored. The Cinch lookup generator reads this checked-in JSON instead of the
host interpreter's table, so updates require an explicit data review and
regeneration. The generated lookup is maintained with the surrounding Cinch
standard library source.
