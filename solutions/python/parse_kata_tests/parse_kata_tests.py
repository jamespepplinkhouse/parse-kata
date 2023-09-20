import pytest
import JsonHelper.jsonHelper as jsonHelp


@pytest.mark.parametrize('testMessage, stringToParse, expected',
                         [('Test string with title first in string',
                           '/type/work	/works/OL10000445W	3	2010-04-28T06:54:19.472104	{"title": "Noir comme un ange", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140866], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000445W", "authors": [{"type": "/type/author_role", "author": {"key": "/authors/OL3965493A"}}], "type": {"key": "/type/work"}, "revision": 3}'
                           ,
                           '{"title": "Noir comme un ange", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140866], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000445W", "authors": [{"type": "/type/author_role", "author": {"key": "/authors/OL3965493A"}}], "type": {"key": "/type/work"}, "revision": 3}')
                          ])
def test_jsonHelper_extract_from_string_returns_valid_json(testMessage, stringToParse, expected):
    actual = jsonHelp.extractJsonFromString(stringToParse)
    assert expected == actual
