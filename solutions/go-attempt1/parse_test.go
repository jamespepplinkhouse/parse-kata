package main

import "testing"

var line = []byte(`/type/work	/works/OL10000049W	3	2010-04-28T06:54:19.472104	{"title": "Le crime organise/quatre films exemplaires", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140091], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000049W", "authors": [{"type": "/type/author_role", "author": {"key": "/a/OL3964979A"}}], "type": {"key": "/type/work"}, "id": 45170289, "revision": 3}`)

// 700 ns/op
func BenchmarkParseLineJson(b *testing.B) {
	for n := 0; n < b.N; n++ {
		ParseLineJson(line)
	}
}

// 45 ns/op
func BenchmarkParseLineRaw(b *testing.B) {
	for n := 0; n < b.N; n++ {
		ParseLineRaw(line)
	}
}
