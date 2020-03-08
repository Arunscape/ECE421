sh = """Shall I compare thee to a summer's day?
Thou art more lovely and more temperate:
Rough winds do shake the darling buds of May,
And summer's lease hath all too short a date:
Sometimes too hot the eye of heaven shines,
And too often is his gold complexion dimm'd:
And every fair from fair sometimes declines,
By chance or natures changing course untrimm'd;
By thy eternal summer shall not fade,
Nor lose possession of that fair thou owest;
Nor shall Death brag thou wander'st in his shade,
When in eternal lines to time thou growest:
So long as men can breathe or eyes can see,
So long lives this and this gives life to thee.
um so here I'm gonna say the word the a bunch of times in the line here"""

for l in filter(lambda y: y[1] != -1, map(lambda x: (x[0], x[1].find('the')),enumerate(sh.split("\n"), start=0))):
    print(l)
