# Credits

Every font bundled in rban is the work of other people. This file lists authorship **as far as it is known**,
taken from the font files themselves and from TAAG's documentation (<https://patorjk.com/software/taag/>, "About" page);
where it is not known, it says so and nothing is made up. If you are the author of a font and want your name
corrected or the font removed, each font's original header is preserved intact inside its file.

## Original idea

rban's original idea comes from **[TAAG (Text to ASCII Art Generator)](https://patorjk.com/software/taag/), by Pat (patorjk)**.
This project is a command-line take on it, built on the fonts he gathered; all the credit for the idea belongs to him.

## Software and formats

- **FIGlet** and the FIGfont format (`.flf`): the name stands for "Frank, Ian and Glenn's letters". Its authors are
  Frank Sheeran, Ian Chai and Glenn Chappell, from the early 90s (TAAG's page only gives the first names;
  the surnames come from general knowledge, not from that page). With contributions from John Cowan (for example the
  `bdf2flf` converter and the `Mnemonic` font) and Christiaan Keet, among others. Full list: <http://www.figlet.org/>.
- **TOIlet** ("The Other Implementation's letters"): a project that set out to replace FIGlet by improving the font
  format (`.tlf`) and adding color filters. It has been inactive for a long time and FIGlet now supports its fonts.
- **TAAG (Text to ASCII Art Generator)**: by **Pat (patorjk)**, <https://patorjk.com/>. The font list and the vast majority of the fonts bundled in rban come from there. Pat also created **figlet.js** and **figlet-cli**
  (<https://github.com/patorjk/figlet.js>) and the font editor <https://patorjk.com/figfont-editor>, and is the one who
  packaged many fonts in FIGlet format (several headers carry his name for conversions or fixes).
- **JavE** (Java Ascii Versatile Editor), <http://www.jave.de>, by Markus Gebhard: appears in the headers of
  several fonts (FIGlet font exporter, fixes).
- **pyfiglet**, <https://github.com/pwaller/pyfiglet> (MIT license): Copyright (c) 2007-2018 Christopher Jones,
  Stefano Rivera, Peter Waller and contributors. Its font collection was used as a source for fonts not offered by TAAG.
- **TheDraw** and the `.tdf` format: a text-art application from the 80s (its author, according to general knowledge
  and not to TAAG, is Ian E. Davis). On ANSI art and these fonts, the artist **Roy/SAC**
  (<https://www.roysac.com>) has written, and the history of **Superior Art Creations (SAC)** is on Wikipedia.
- `src/tdf.rs` is a reader written for this project whose logic was ported from TAAG's client code; no explicit license was seen for that code.

## Font categories

- **FIGlet** (bundled FIGlet fonts): the classic ones, from the FIGlet community. Authorship is in the table below.
- **ANSI FIGlet** (14 fonts, listed in the `[gradient]` section of fonts.conf): FIGlet versions that **Pat (patorjk) made** of a handful of TheDraw fonts.
  Their headers point to the archive <https://web.archive.org/web/20120819044459/http://www.roysac.com/thedrawfonts-tdf.asp>;
  the original author is only recorded for some (for example `Electronic`), the rest say "Font Author: ?".
- **Removed fonts:** the TAAG collection also has "AOL macro" fonts (drawn for the proportional Arial font, so they come
  out misaligned in a terminal), the Commodore 64 fonts (whose lowercase letters come out as garbage), and some symbolic or broken
  fonts. They are not bundled in rban.
- **TOIlet** (5 fonts, `.tlf` files).
- **TheDraw** (`ansi/`, 3481 colored fonts): the `ansi/tdf-chunk-*.tdf` files **contain no author or license**;
  each font only stores its name, so their authorship cannot be attributed from here. The name suffixes
  (`Cya`, `Gre`, `Pur`, `Red`, `Sil`...) are color variants of the same font, not authors.
  If you know the origin of any of them, add it here.

## FIGlet font table

The attribution is what each file's header says, trimmed to two lines and without email addresses or phone numbers (they remain in the original file). Some headers cite licenses: the ones converted from
X11 BDF fonts (`5x7`, `5x8`, etc.) keep the X Consortium copyright notice, which must be preserved.

Total: 386 fonts (14 ANSI FIGlet, 367 FIGlet, 5 TOIlet); 27 whose header does not mention an author
(they are listed anyway, with no data; in some cases the header identifies the author only by email address, which is omitted here).

1Row.flf (FIGlet)
  1row font by unknown
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

3-D.flf (FIGlet)
  3-D font created by Daniel Henninger
  Font modified June 17, 2007 by patorjk

3D Diagonal.flf (FIGlet)
  Author : nabis, LG Beard, Markus Gebhard and others
  This font has been created using JavE's FIGlet font export assistant.

3D-ASCII.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

3x5.flf (FIGlet)
  3x5 font by Richard Kirk .
  by Daniel Cabeza Gras

4Max.flf (FIGlet)
  4max.flf by Philip Menke

5 Line Oblique.flf (FIGlet)
  _(header does not mention an author)_

5x7.flf (FIGlet)
  Converted from 5x7.bdf by bdf2flf (by John Cowan )
  Copyright (c) 1991 X Consortium

5x8.flf (FIGlet)
  Converted from 5x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Cognition Corp.

6x10.flf (FIGlet)
  Converted from 6x10.bdf by bdf2flf (by John Cowan )
  Upper half by gildea April 1994

6x9.flf (FIGlet)
  Converted from 6x9.bdf by bdf2flf (by John Cowan )
  Copyright (c) 1989 X Consortium

Acrobatic.flf (FIGlet)
  Acrobatic by Randy Ransom 8/94

Alligator.flf (FIGlet)
  Alligator by Simon Bradley

Alligator2.flf (FIGlet)
  Alligator2 by Daniel Wiz. AKA Merlin Greywolf

Alpha.flf (FIGlet)
  Alpha font by Lennert Stock
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

Alphabet.flf (FIGlet)
  Alphabet by Wendell Hicken 11/93
  Font modified June 17, 2007 by patorjk

AMC 3 Line.flf (FIGlet)
  3 lines tall font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC 3 Liv1.flf (FIGlet)
  3 line vertical font (tilt head to left) font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC AAA01.flf (FIGlet)
  AMC Alt.Ascii-Art font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Neko.flf (FIGlet)
  Nekochan thin font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Razor.flf (FIGlet)
  Razor font by LESTER
  > The "G" was rediddled masterfully by: (Tom R. Earlywine)

AMC Razor2.flf (FIGlet)
  AMC razor 2 font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Slash.flf (FIGlet)
  AMCslash font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Slider.flf (FIGlet)
  Slider font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Thin.flf (FIGlet)
  Thin font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Tubes.flf (FIGlet)
  AMCtubes font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

AMC Untitled.flf (FIGlet)
  AMC untitled font by LESTER
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

ANSI Compact.flf (ANSI FIGlet)
  Font Author: Loic Cressot
  FIGFont created with: http://patorjk.com/figfont-editor

ANSI Regular.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figlet-editor

ANSI Shadow.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

Arrows.flf (FIGlet)
  ARROWS by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

ASCII 12.flf (FIGlet)
  _(header does not mention an author)_

ASCII 9.flf (FIGlet)
  _(header does not mention an author)_

ASCII New Roman.flf (FIGlet)
  Author : Marcin Glinsky and others
  Font ascii_new_roman, created by Marcin Glinski

Avatar.flf (FIGlet)
  Avatar by Claude Martins 02/95

B1FF.flf (FIGlet)
  B1FF by Joe Rumsey 12/94 based on Terminal by Glenn Chappell

Babyface Lame.flf (FIGlet)
  BABYFACE Lame by cat, v0.8 2019

Babyface Leet.flf (FIGlet)
  BABYFACE Leet by cat, v0.8 2019

Banner.flf (FIGlet)
  banner.flf version 2 by Ryan Youck
  Katakana characters by Vinney Thai

Banner3-D.flf (FIGlet)
  banner3-D by Merlin Greywolf

Banner3.flf (FIGlet)
  banner3 by Merlin Greywolf

Banner4.flf (FIGlet)
  banner4 by Merlin Greywolf

Barbwire.flf (FIGlet)
  BARBWIRE by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Basic.flf (FIGlet)
  basic.flf by Craig O'Flaherty
  Font modified June 17, 2007 by patorjk

Bear.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Bell.flf (FIGlet)
  Lower case letters, punctuation, and miscellaneous changes by Kent Nassen,
  Upper case letters originally by Joshua Bell:

Benjamin.flf (FIGlet)
  _(header does not mention an author)_

Big ASCII 12.flf (FIGlet)
  _(header does not mention an author)_

Big ASCII 9.flf (FIGlet)
  _(header does not mention an author)_

Big Chief.flf (FIGlet)
  _(header does not mention an author)_

Big Money-ne.flf (FIGlet)
  > bigmoney-ne : by nathan bloomfield
  Font modified June 17, 2007 by patorjk

Big Money-nw.flf (FIGlet)
  > bigmoney-nw : by nathan bloomfield
  Font modified June 17, 2007 by patorjk

Big Money-se.flf (FIGlet)
  > bigmoney-se : by nathan bloomfield
  Font modified June 17, 2007 by patorjk

Big Money-sw.flf (FIGlet)
  > bigmoney-sw : by nathan bloomfield
  Font modified June 17, 2007 by patorjk

Big Mono 12.flf (FIGlet)
  _(header does not mention an author)_

Big Mono 9.flf (FIGlet)
  _(header does not mention an author)_

Big.flf (FIGlet)
  Big by Glenn Chappell 4/93 -- based on Standard
  Greek characters by Bruce Jakeway

Bigfig.flf (FIGlet)
  Expansion font for bigfig program by Glenn Chappell 4/93 -- based on Mini
  bigfig" is available by anonymous ftp from "ftp.nicoh.com". Look in

Block.flf (FIGlet)
  Block by Glenn Chappell 4/93 -- straight version of Lean
  Modified by Paul Burton 12/96 to include new parameter

Blocks.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

blocky.flf (FIGlet)
  blocky by Mike Kinney

Bloody.flf (ANSI FIGlet)
  Figlet conversion by patorjk, April 17, 2008

BlurVision ASCII.flf (FIGlet)
  Font Author: Aiden Neuding
  FIGFont created with: http://patorjk.com/figfont-editor

Bolger.flf (FIGlet)
  fatty.flf by Mike Rosulek. 1/12/95
  Font modified June 17, 2007 by patorjk

Braced.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Bright.flf (FIGlet)
  Bright Font by Dennis Monk 7/94

brite.flf (FIGlet)
  Converted from brite.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

briteb.flf (FIGlet)
  Converted from briteb.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

britebi.flf (FIGlet)
  Converted from britebi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

britei.flf (FIGlet)
  Converted from britei.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

Broadway KB.flf (FIGlet)
  Author :myflix
  This font has been created using JavE's FIGlet font export assistant.

Broadway.flf (FIGlet)
  Figlet translation by Kent Nassen, , 1/2/95

Bubble.flf (FIGlet)
  Bubble by Glenn Chappell 4/93
  Enhanced for Latin-2,3,4 by John Cowan

Bulbhead.flf (FIGlet)
  Bulbhead by Jef Poskanzer, 23jun94
  Update February 12, 2002 by Markus Gebhard

calgphy2.flf (FIGlet)
  Modified by Paul Burton September 96 from caligraphy.flf as a demonstration of
  Original caligraphy.flf composed by Vinney Thai October 94

Caligraphy.flf (FIGlet)
  caligraphy.flf composed by Vinney Thai
  caligraphy font (J, O, R) composed by Jorn

Caligraphy2.flf (FIGlet)
  Modified by Paul Burton September 96 from caligraphy.flf as a demonstration of
  Original caligraphy.flf composed by Vinney Thai October 94

Calvin S.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

Cards.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Catwalk.flf (FIGlet)
  CATWALK by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

chartr.flf (FIGlet)
  Converted from chartr.bdf by bdf2flf (by John Cowan )
  Copyright 1988 Bitstream, Inc., Cambridge, Massachusetts, USA

chartri.flf (FIGlet)
  Converted from chartri.bdf by bdf2flf (by John Cowan )
  Copyright 1988 Bitstream, Inc., Cambridge, Massachusetts, USA

Chiseled.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Chunky.flf (FIGlet)
  Square by Chris Gill, 30-JUN-94 -- based on .sig of Jeb Hagan.
  Font modified June 17, 2007 by patorjk

Circle.flf (FIGlet)
  This is circle.tlf, or “Circle”, by Sam Hocevar . It was
  created on October 1st, 2006.

Classy.flf (ANSI FIGlet)
  Font Author: Loic Cressot
  FIGFont created with: http://patorjk.com/figfont-editor

clb6x10.flf (FIGlet)
  Converted from clb6x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clb8x10.flf (FIGlet)
  Converted from clb8x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clb8x8.flf (FIGlet)
  Converted from clb8x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

cli8x8.flf (FIGlet)
  Converted from cli8x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr4x6.flf (FIGlet)
  Converted from clr4x6.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr5x10.flf (FIGlet)
  Converted from clr5x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr5x6.flf (FIGlet)
  Converted from clr5x6.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr5x8.flf (FIGlet)
  Converted from clr5x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr6x10.flf (FIGlet)
  Converted from clr6x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr6x6.flf (FIGlet)
  Converted from clr6x6.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr6x8.flf (FIGlet)
  Converted from clr6x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr7x10.flf (FIGlet)
  Converted from clr7x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr7x8.flf (FIGlet)
  Converted from clr7x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr8x10.flf (FIGlet)
  Converted from clr8x10.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

clr8x8.flf (FIGlet)
  Converted from clr8x8.bdf by bdf2flf (by John Cowan )
  Copyright 1989 Dale Schumacher,

Coder Mini.flf (ANSI FIGlet)
  Font Author: Loic Cressot
  FIGFont created with: http://patorjk.com/figfont-editor

Coinstak.flf (FIGlet)
  COINSTAK by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Cola.flf (FIGlet)
  Author : MikeChat
  This font has been created using JavE's FIGlet font export assistant.

Colossal.flf (FIGlet)
  Font modified June 17, 2007 by patorjk

Computer.flf (FIGlet)
  computer.flf by Mike Rosulek , 7/12/95. Check out my

Contessa.flf (FIGlet)
  Contessa by Christopher Joseph Pirillo

Contrast.flf (FIGlet)
  Contrast Font by Dennis Monk 7/94
  Original font, I think. Maybe inspired by one of Scarecrow's .sigs.

cosmic.flf (FIGlet)
  COSMIC.FLF by Mike Rosulek , 7/11/95. Bugs fixed 7/13/95.

Cosmike.flf (FIGlet)
  cosmic.flf by Mike Rosulek , 7/11/95. Check out my
  Modified and corrected by Michel Eftimakis

Cosmike2.flf (FIGlet)
  Font Author: half Cosmike, half Darth ObiKy
  FIGFont created with: http://patorjk.com/figfont-editor

cour.flf (FIGlet)
  Converted from cour.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

courb.flf (FIGlet)
  Converted from courb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

courbi.flf (FIGlet)
  Converted from courbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

couri.flf (FIGlet)
  Converted from couri.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

Crawford.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 1/2/95

Crawford2.flf (FIGlet)
  Font created from original Crawford, update by PAT or JK, Jan 2008
  Figlet conversion by Kent Nassen, , 1/2/95

Crazy.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Cricket.flf (FIGlet)
  Cricket by Leslie Bates Jan. 1, 1996
  well as Chris Gill who designed the Square font, some of which was used in

Cursive.flf (FIGlet)
  Based on cursive(6) by Jan Wolter on 24 Jul 1985
  Figlet-ized and by Wendell Hicken on 5 Mar 1994

Cyberlarge.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 8-10-94

Cybermedium.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 8-11-94

Cybersmall.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 8-11-94

Cygnet.flf (FIGlet)
  Author : Christian 'CeeJay' Jensen
  Cygnet" - A figfont by CeeJay

DANC4.flf (FIGlet)
  DANC4 by Richard Sabey 9.2003
  The cipher ignores punctuation, and marks a space by a flag held by the

Dancing Font.flf (FIGlet)
  Author : Myflix
  This font has been created using JavE's FIGlet font export assistant.

Def Leppard.flf (FIGlet)
  Def Leppard by Hanspeter Niederstrasser -- based on Standard
  Font modified June 17, 2007 by patorjk

Delta Corps Priest 1.flf (ANSI FIGlet)
  Font Author: CoSMiC cHiLD
  FIGFont created by patorjk.com's FIGFont Editor: http://patorjk.com/figfont-editor

DiamFont.flf (FIGlet)
  Font Author: Diamond Planet
  FIGFont created with: http://patorjk.com/figfont-editor

Diamond.flf (FIGlet)
  DIAMOND by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Digital.flf (FIGlet)
  Digital by Glenn Chappell 1/94 -- based on Bubble
  Enhanced for Latin-2,3,4 by John Cowan

Doh.flf (FIGlet)
  doh.flf by Curtis Wanner

Doom.flf (FIGlet)
  DOOM by Frans P. de Vries 18 Jun 1996
  based on Big by Glenn Chappell 4/93 -- based on Standard

DOS Rebel.flf (FIGlet)
  Rebel by Valerie Mates , based on a font by Ron Bliss
  (who sometimes goes by the name "rebel" because his initials are REB).

Dot Matrix.flf (FIGlet)
  dotmatrix.flf by Curtis Wanner

Double Shorts.flf (FIGlet)
  Author : myflix
  This font is a variation of double designed by Kent Nassen 11/24/94

Double.flf (FIGlet)
  figlet conversion by Kent Nassen, , 11/24/94

double_blocky.flf (FIGlet)
  Font Author: CatPieLeaf
  FIGFont created with:

Dr Pepper.flf (FIGlet)
  Author: Eero Tamminen, .

Efti Font.flf (FIGlet)
  This font is (c) Michel Eftimakis 1995 -- Version 2.0 -- 27 jan 1995

Efti Italic.flf (FIGlet)
  This font is (c) Michel Eftimakis 1995 -- Version 1.0 -- 27 jan 1995

Efti Piti.flf (FIGlet)
  This font is (c) Michel Eftimakis 1995 -- Version 1.1 -- 15 feb 1995

Efti Robot.flf (FIGlet)
  This font is (c) Michel Eftimakis 1995 -- Version 1.0 -- 11 may 1995
  Font modified June 17, 2007 by patorjk

eftitalic.flf (FIGlet)
  This font is (c) Michel Eftimakis 1995 -- Version 1.0 -- 27 jan 1995

Electronic.flf (ANSI FIGlet)
  Font Author: Derek Lemay <THe PHaRCYDe>
  FIGFont created with: http://patorjk.com/figfont-editor

Elite.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

Emboss 2.flf (FIGlet)
  This is emboss.tlf, or “Emboss”, by Sam Hocevar . It was
  created on September 30th, 2006.

Emboss.flf (FIGlet)
  This is emboss.tlf, or “Emboss”, by Sam Hocevar . It was
  created on September 30th, 2006.

Epic.flf (FIGlet)
  Epic by Claude Martins 12/94

Fender.flf (FIGlet)
  Fender by Scooter 8/94

Filter.flf (FIGlet)
  Author : Aaron Nolan
  This font has been created using JavE's FIGlet font export assistant.

Fire Font-k.flf (FIGlet)
  Author : MJP
  This font has been created using JavE's FIGlet font export assistant.

Fire Font-s.flf (FIGlet)
  Author : MJP
  This font has been created using JavE's FIGlet font export assistant.

Flower Power.flf (FIGlet)
  This font has been created using JavE's FIGlet font export assistant.
  Font modified June 17, 2007 by patorjk

Font Font.flf (FIGlet)
  Font Author: Oscar Cole
  FIGFont created with: http://patorjk.com/figfont-editor

Four Tops.flf (FIGlet)
  Fourtops by Randall Ransom 4/94

Fraktur.flf (FIGlet)
  Fraktur.flf by Philip Menke

Fun Face.flf (FIGlet)
  Author : MJP
  This font has been created using JavE's FIGlet font export assistant.

Fun Faces.flf (FIGlet)
  Author : MJP (AKA MikeChat)
  This font has been created using JavE's FIGlet font export assistant.

Future Smooth.flf (FIGlet)
  This is "Future Smooth" (Future Smooth.flf), based on "Future" (future.tlf) by Sam Hocevar. This variant uses arcs instead of sharp corners to smooth out the font. Bec...

Future Thin.flf (FIGlet)
  This is "Future Thin" (Future Thin.flf), based on "Future" (future.tlf) by Sam Hocevar. This variant uses light box drawing characters for a thinner look.
  permitted...

Future.flf (FIGlet)
  This is future.tlf, or “Future”, by Sam Hocevar.
  permitted by applicable law. You can redistribute it and/or modify it under

Fuzzy.flf (FIGlet)
  fuzzy.flf by Juan Car

Georgi16.flf (FIGlet)
  Georgia 16 by Richard Sabey 8.2003
  Includes new parameter supported by FIGlet and FIGWin.

Georgia11.flf (FIGlet)
  Georgia 11 by Richard Sabey 9.2003
  Includes new parameter supported by FIGlet and FIGWin.

Ghost.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Ghoulish.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Glenyn.flf (FIGlet)
  Author : Lukasz Tyrala (lt.)
  ..xx.. (C) COPYRIGHTS MMIV by

Goofy.flf (FIGlet)
  Written by Steven <GoofY> de Brouwer
  This font is NOT designed to be used for clear, long text!

Gothic.flf (FIGlet)
  Gothic.flf - Taken from a banner program called "gothic" by Howard Chu
  Figletized by Wendell Hicken 11/19/93

Graceful.flf (FIGlet)
  Graceful-6x4 by Mikhael Goikhman, http://migo.n3.net/, 20/Jan/2002.

Gradient.flf (FIGlet)
  Gradient.flf by Philip Menke
  Concept by Bob Allison Font by Philip Menke

Graffiti.flf (FIGlet)
  This figlet font designed by Leigh Purdie
  'fig-fonted' by Leigh Purdie and Tim Maggio

Heart Left.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Heart Right.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

helv.flf (FIGlet)
  Converted from helv.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

helvb.flf (FIGlet)
  Converted from helvb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

helvbi.flf (FIGlet)
  Converted from helvbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

helvi.flf (FIGlet)
  Converted from helvi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

Henry 3D.flf (FIGlet)
  Characters by Henry Segerman ,
  Converted to FIGlet font by Markus Gebhard

Hollywood.flf (FIGlet)
  hollywood.flf by Juan Car

Horizontal Left.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Horizontal Right.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Impossible.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Invita.flf (FIGlet)
  _(header does not mention an author)_

Isometric1.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the fonts posted by Lennert Stock:

Isometric2.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the fonts posted by Lennert Stock:

Isometric3.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the post by Lennert Stock:

Isometric4.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the fonts posted by Lennert Stock:

Italic.flf (FIGlet)
  by: Bas Meijer
  fixed by: Ryan Youck

Jacky.flf (FIGlet)
  Jacky font by Meph.'99

Jazmine.flf (FIGlet)
  _(header does not mention an author)_

JS Block Letters.flf (FIGlet)
  Block Letters By Joan Stark
  Figlet conversion by patorjk, April 17, 2008

JS Bracket Letters.flf (FIGlet)
  Bracket Letters By Joan Stark
  Figlet conversion by patorjk, April 17, 2008

JS Capital Curves.flf (FIGlet)
  Author :
  This font has been created using JavE's FIGlet font export assistant.

JS Cursive.flf (FIGlet)
  Cursive By Joan Stark
  Figlet conversion by patorjk, April 17, 2008

JS Stick Letters.flf (FIGlet)
  Stick Letters By Joan Stark
  Figlet conversion by patorjk, April 17, 2008

Kban.flf (FIGlet)
  Kban by Randy Jae Weinstein

Keyboard.flf (FIGlet)
  keyboard.flf composed by Vinney Thai

Knob.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Konto Slant.flf (FIGlet)
  Author : David Dahlberg
  Font designed by:

Konto.flf (FIGlet)
  Author : Markus Gebhard
  This font has been created using Jave's FIGlet font export assistant

Larry 3D 2.flf (FIGlet)
  larry3d.flf by Larry Gelberg
  tweaked by Glenn Chappell

Larry 3D.flf (FIGlet)
  larry3d.flf by Larry Gelberg
  tweaked by Glenn Chappell

LCD.flf (FIGlet)
  Lcd by Karl von Laudermann 11/93

Lean.flf (FIGlet)
  Lean by Glenn Chappell 4/93 -- based on various .sig's
  Modified by Paul Burton 12/96 to include new parameter

Letter.flf (FIGlet)
  This is letter.tlf, or “Letter”, Copyright (c) 2008 Francesco Poli
  permitted by applicable law. You can redistribute it and/or modify it under

Letters.flf (FIGlet)
  Letters by Sriram J. Gollapalli, July 10, 1994
  Font modified June 17, 2007 (without permission) by patorjk

Lil Devil.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Line Blocks.flf (FIGlet)
  Author : Bateau (lbm)
  This font has been created using JavE's FIGlet font export assistant.

Linguaholic Mini Block.flf (FIGlet)
  Linguaholic Mini Block — an original FIGlet font by Linguaholic, 2026.
  Copyright (c) 2026 Linguaholic (https://linguaholic.com)

Linguaholic Neon.flf (FIGlet)
  Linguaholic Neon — an original FIGlet font by Linguaholic, 2026.
  Copyright (c) 2026 Linguaholic (https://linguaholic.com)

Linguaholic Rounded.flf (FIGlet)
  Linguaholic Rounded — an original FIGlet font by Linguaholic, 2026.
  Copyright (c) 2026 Linguaholic (https://linguaholic.com)

Linguaholic Shadow 3D.flf (FIGlet)
  Linguaholic Shadow 3D — an original FIGlet font by Linguaholic, 2026.
  Copyright (c) 2026 Linguaholic (https://linguaholic.com)

Linux.flf (FIGlet)
  linux by Larry Smith 5/12/95 - based on .sig of Roberth Karman.

Lockergnome.flf (FIGlet)
  LockerGnome by Christopher Joseph Pirillo
  Figletized by Simon Bradley

Madrid.flf (FIGlet)
  madrid.flf by Juan Car

Marquee.flf (FIGlet)
  MARQUEE by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Maxfour.flf (FIGlet)
  Maxfour by Randall Ransom 1/94

Merlin1.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Merlin2.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Mike.flf (FIGlet)
  Mike by Michael Sullivan 1994
  Figlet version of mikewriting generated by hand...

Mini.flf (FIGlet)
  Mini by Glenn Chappell 4/93
  Modified by Paul Burton 12/96 to include new parameter

miniwi.flf (FIGlet)
  _(header does not mention an author)_

Mnemonic.flf (FIGlet)
  mnemonic - a FIGfont by John Cowan
  are prefixed by &; larger mnemonics are prefixed by &_ and followed by _.

Modular.flf (FIGlet)
  Author :myflix
  This font has been created using JavE's FIGlet font export assistant.

Mono 12.flf (FIGlet)
  _(header does not mention an author)_

Mono 9.flf (FIGlet)
  _(header does not mention an author)_

Moscow.flf (FIGlet)
  Cyrillic font by Tracy Schuhwerk .

Muzzle.flf (FIGlet)
  Author : Katerina L.
  This font has been created using JavE's FIGlet font export assistant.

Nancyj-Fancy.flf (FIGlet)
  Modified to make tabs blanks by Paul Burton

Nancyj-Improved.flf (FIGlet)
  _(header does not mention an author)_

Nancyj-Underlined.flf (FIGlet)
  _(header does not mention an author)_

Nancyj.flf (FIGlet)
  _(header does not mention an author)_

Nipples.flf (FIGlet)
  NIPPLES by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

NScript.flf (FIGlet)
  nscript by Normand Veilleux Oct 7, 1994
  figletized by Peter Samuelson Nov 29, 1994

NV Script.flf (FIGlet)
  Script by Normand Veilleux
  FIGlet Font compiled by Jerrad Pierce

O8.flf (FIGlet)
  Original font by Gordon Lee
  Original figletization by Tony Nugent

Ogre.flf (FIGlet)
  Standard by Glenn Chappell & Ian Chai 3/93 -- based on .sig of Frank Sheeran

Old Banner.flf (FIGlet)
  banner.flf by Ryan Youck

OS2.flf (FIGlet)
  By Kent Nassen, , 4/30/95.

Pagga.flf (FIGlet)
  This is pagga.tlf, or “Pagga”, by Sam Hocevar and pagga.
  permitted by applicable law. You can redistribute it and/or modify it under

Patorjk's Cheese.flf (FIGlet)
  Patorjk's Cheese" font by patorjk and x98.

Patorjk-HeX.flf (FIGlet)
  Patorjk-HeX" font by patorjk and HeX.

Pawp.flf (FIGlet)
  pawp.flf by Curtis Wanner
  (my first figlet font. It is based on puffy.flf by Juan Car)

Peaks Slant.flf (FIGlet)
  peaksslant.flf (C) 1994 by Victor Parada 94/09/23.

Peaks.flf (FIGlet)
  PEAKS by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Pebbles.flf (FIGlet)
  Pebbles, figletized by Empath 7/94
  completed by Ryan Youck 8/94

Pepper.flf (FIGlet)
  pepper.flf by Dr. Pepper
  Completed and ported to figlet by Juan Car

Poison.flf (FIGlet)
  poison.flf composed into figlet by Vinney Thai
  poison font (numbers & puntuation marks) composed by Vinney Thai

Puffy.flf (FIGlet)
  puffy.flf by Juan Car

Puzzle.flf (FIGlet)
  Author : myflix and Mikechat
  This font has been created using JavE's FIGlet font export assistant.

Pyramid.flf (FIGlet)
  Pyramid by Claude Martins 02/95

Rammstein.flf (FIGlet)
  Author : Bateau (lbm)
  This font has been created using JavE's FIGlet font export assistant.

Rebel.flf (FIGlet)
  Rebel by Valerie Mates , based on a font by Ron Bliss
  (who sometimes goes by the name "rebel" because his initials are REB).

Rectangles.flf (FIGlet)
  rectangles.flf by David Villegas 12/94

Red Phoenix.flf (FIGlet)
  Red Phoenix" font by Red Phoenix
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

rev.flf (FIGlet)
  reverse.flf by Matt E. Thurston

Reverse.flf (FIGlet)
  reverse.flf by Matt E. Thurston

Roman.flf (FIGlet)
  Roman by Nick Miners
  Font modified June 17, 2007 by patorjk

Rounded.flf (FIGlet)
  Rounded by Nick Miners

Rowan Cap.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 1/1/95

Rozzo.flf (FIGlet)
  rozzo.flf font by Mike Rosulek [] 1/12/95

RubiFont.flf (FIGlet)
  Font Author: RubixTW
  FIGFont created with: http://patorjk.com/figfont-editor

S Blood.flf (FIGlet)
  Figlet conversion by Kent Nassen based on the
  font designed by:

sans.flf (FIGlet)
  Converted from sans.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

sansb.flf (FIGlet)
  Converted from sansb.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

sansbi.flf (FIGlet)
  Converted from sansbi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

sansi.flf (FIGlet)
  Converted from sansi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

Santa Clara.flf (FIGlet)
  Derived from a copyrighted program by Jan Wolter
  Figlet-ized by Wendell Hicken

sbook.flf (FIGlet)
  Converted from sbook.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

sbookb.flf (FIGlet)
  Converted from sbookb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

sbookbi.flf (FIGlet)
  Converted from sbookbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

sbooki.flf (FIGlet)
  Converted from sbooki.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

Script.flf (FIGlet)
  Script by Glenn Chappell 4/93
  Modified by Paul Burton 12/96 to include new parameter

Serifcap.flf (FIGlet)
  serifcap by Bruce M. Binder

Shadow.flf (FIGlet)
  Shadow by Glenn Chappell 6/93 -- based on Standard & SmShadow
  Modified by Paul Burton 12/96 to include new parameter

Shimrod.flf (FIGlet)
  HHscript by Shimrod (Herman Hiddema), based on his own .sig

Short.flf (FIGlet)
  Short and sweet by Sub-Zero
  Completed and ported to figlet by Juan Car

SL Script.flf (FIGlet)
  Based on cursive(6) by Jan Wolter on 24 Jul 1985
  Figlet-ized and by Wendell Hicken on 5 Mar 1994

Slant.flf (FIGlet)
  Slant by Glenn Chappell 3/93 -- based on Standard
  Modified by Paul Burton 12/96 to include new parameter

Slide.flf (FIGlet)
  slide.flf (C) 1994 by Victor Parada 94/08/18.

Small ASCII 12.flf (FIGlet)
  _(header does not mention an author)_

Small ASCII 9.flf (FIGlet)
  _(header does not mention an author)_

Small Block.flf (FIGlet)
  This is smblock.tlf, or “Small Block”, by Sam Hocevar.
  permitted by applicable law. You can redistribute it and/or modify it under

Small Caps.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Small Isometric1.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the fonts posted by Lennert Stock:

Small Keyboard.flf (FIGlet)
  Adapted for figlet by Kent Nassen, , 11/22/94, based

Small Mono 12.flf (FIGlet)
  _(header does not mention an author)_

Small Mono 9.flf (FIGlet)
  _(header does not mention an author)_

Small Poison.flf (FIGlet)
  smpoison.flf figletized by Vinney Thai

Small Script.flf (FIGlet)
  SmScript by Glenn Chappell 4/93 -- based on Script
  Modified by Paul Burton 12/96 to include new parameter

Small Shadow.flf (FIGlet)
  SmShadow by Glenn Chappell 4/93 -- based on Small
  Modified by Paul Burton 12/96 to include new parameter

Small Slant.flf (FIGlet)
  SmSlant by Glenn Chappell 6/93 - based on Small & Slant
  Modified by Paul Burton 12/96 to include new parameter

Small.flf (FIGlet)
  Small by Glenn Chappell 4/93 -- based on Standard
  Modified by Paul Burton 12/96 to include new parameter

smascii12.tlf (TOIlet)
  _(header does not mention an author)_

smascii9.tlf (TOIlet)
  _(header does not mention an author)_

smblock.tlf (TOIlet)
  This is smblock.tlf, or “Small Block”, by Sam Hocevar.
  permitted by applicable law. You can redistribute it and/or modify it under

smisome1.flf (FIGlet)
  Figlet conversion by Kent Nassen , 8-10-94, based
  on the fonts posted by Lennert Stock:

smkeyboard.flf (FIGlet)
  Adapted for figlet by Kent Nassen, , 11/22/94, based

smmono12.tlf (TOIlet)
  _(header does not mention an author)_

smmono9.tlf (TOIlet)
  _(header does not mention an author)_

smscript.flf (FIGlet)
  SmScript by Glenn Chappell 4/93 -- based on Script
  Modified by Paul Burton 12/96 to include new parameter

smshadow.flf (FIGlet)
  SmShadow by Glenn Chappell 4/93 -- based on Small
  Modified by Paul Burton 12/96 to include new parameter

smslant.flf (FIGlet)
  SmSlant by Glenn Chappell 6/93 - based on Small & Slant
  Modified by Paul Burton 12/96 to include new parameter

Soft.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Speed.flf (FIGlet)
  Speed by Claude Martins 2/95 -- based on Slant

Spliff.flf (FIGlet)
  > spliff : by nathan bloomfield
  > based on the letters a,i,j,m,n,r,u, seen on the web, by E.DUB

Stacey.flf (FIGlet)
  Figlet conversion by Kent Nassen, , 11/24/94

Stampate.flf (FIGlet)
  _(header does not mention an author)_

Stampatello.flf (FIGlet)
  _(header does not mention an author)_

Standard.flf (FIGlet)
  Standard by Glenn Chappell & Ian Chai 3/93 -- based on Frank's .sig
  Modified for figlet 2.2 by John Cowan

Star Strips.flf (FIGlet)
  Author : Ralph Bluecoat
  This font has been created using JavE's FIGlet font export assistant.

Star Wars.flf (FIGlet)
  starwars.flf by Ryan Youck Dec 25/1994
  Based on Big.flf by Glenn Chappell

Stellar.flf (FIGlet)
  STELLAR by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Stforek.flf (FIGlet)
  Author: Marcin `stforek` Glinski
  This font has been created using JavE's FIGlet font

Stick Letters.flf (FIGlet)
  Stick Figures By Joan Stark
  Figlet conversion by patorjk, April 17, 2008

Stop.flf (FIGlet)
  Stop by David Walton
  Derived from Rounded by Nick Miners

Straight.flf (FIGlet)
  by: Bas Meijer
  fixed by: Ryan Youck

Stronger Than All.flf (ANSI FIGlet)
  Font Author: pC wIZARd
  Font modified by patorjk to add X character

Sub-Zero.flf (FIGlet)
  Sub-Zero" font by Sub-Zero
  > Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)

Swamp Land.flf (FIGlet)
  Author : bpg
  made for fun (ty myflix and LGB for help)

Swan.flf (FIGlet)
  Author : Christian 'CeeJay' Jensen
  Swan" - A figfont by CeeJay

Sweet.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Tanja.flf (FIGlet)
  Tanja was written for -=Tanja Lynnx Jackson=- by

Technologizer.flf (FIGlet)
  Font Author: Igor Zevel
  FIGFont created with: http://patorjk.com/figfont-editor

Term.flf (FIGlet)
  Terminal by Glenn Chappell 4/93
  Enhanced for Latin-2,3,4 by John Cowan

Terrace.flf (FIGlet)
  a FIGFont by AJN
  created with: http://patorjk.com/figfont-editor

Test1.flf (FIGlet)
  Author :
  This font has been created using Jave's FIGlet font export assistant.

The Edge.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

Thick.flf (FIGlet)
  Thick by Randall Ransom 2/94

Thin.flf (FIGlet)
  Based on a lowercase font posted by
  Figletized by Wendell Hicken 11/93

THIS.flf (ANSI FIGlet)
  Font Author: ?
  FIGFont created with: http://patorjk.com/figfont-editor

Thorned.flf (FIGlet)
  Author :
  This font has been created using JavE's FIGlet font export assistant.

Three Point.flf (FIGlet)
  threepoint by Randall Ransom 4/94

Tiles.flf (FIGlet)
  TILES by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

times.flf (FIGlet)
  Converted from times.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

Tinker-Toy.flf (FIGlet)
  Tinker-toy by Wendell Hicken 11/93

Tmplr.flf (FIGlet)
  Font Author: Eugene Ghanizadeh Khoub
  FIGFont created with: http://patorjk.com/figfont-editor

Tombstone.flf (FIGlet)
  About Today's Cryptography," v2.0 draft 2f, 9/20/93, by Paul Fahn,
  copyright 1993, RSA Laboratories

Train.flf (FIGlet)
  Author :myflix
  This font has been created using JavE's FIGlet font export assistant.

Trek.flf (FIGlet)
  // 1994, by Peter L. Buschman //
  // Please send all comments to the author at: //

tty.flf (FIGlet)
  Converted from tty.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright Bigelow & Holmes 1986, 1985.

ttyb.flf (FIGlet)
  Converted from ttyb.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright Bigelow & Holmes 1986, 1985.

Tubular.flf (FIGlet)
  TUBULAR by Ron Fritz 8/94
  Font modified June 17, 2007 by patorjk

Twisted.flf (FIGlet)
  Author : LG Beard
  This font has been created using JavE's FIGlet font export assistant.

Two Point.flf (FIGlet)
  twopoint by Bruce Jakeway

Univers.flf (FIGlet)
  Univers by Normand Veilleux
  figletized by Glenn Chappell January 12, 1994

USA Flag.flf (FIGlet)
  Figlet conversion by Kent Nassen based on the
  font design by:

utopia.flf (FIGlet)
  Converted from utopia.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright (c) 1989, 1991 Adobe Systems Incorporated. All rights reserved.

utopiab.flf (FIGlet)
  Converted from utopiab.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright (c) 1989, 1991 Adobe Systems Incorporated. All rights reserved.

utopiabi.flf (FIGlet)
  Converted from utopiabi.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright (c) 1989, 1991 Adobe Systems Incorporated. All rights reserved.

utopiai.flf (FIGlet)
  Converted from utopiai.bdf by bdf2flf (by John Cowan )
  COPYRIGHT "Copyright (c) 1989, 1991 Adobe Systems Incorporated. All rights reserved.

Varsity.flf (FIGlet)
  Author : myflix
  This font has been created using JavE's FIGlet font export assistant.

Wavescape.flf (FIGlet)
  Font Author: SCA
  FIGFont created with: http://patorjk.com/figfont-editor

Wavy.flf (FIGlet)
  Wavy by Brian Krog 10/05

Weird.flf (FIGlet)
  by: Bas Meijer
  fixed by: Ryan Youck

Wet Letter.flf (FIGlet)
  Author : MJP
  This font has been created using JavE's FIGlet font export assistant.

Whimsy.flf (FIGlet)
  Figlet conversion by Kent Nassen,

WideTerm.flf (FIGlet)
  This is wideterm.tlf, or “WideTerm”, by Sam Hocevar . It was
  created on August 2nd, 2007.

Wow.flf (FIGlet)
  Author : Pipeline
  This font was inspired by pwning nubs in WoW

xbrite.flf (FIGlet)
  Converted from xbrite.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xbriteb.flf (FIGlet)
  Converted from xbriteb.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xbritebi.flf (FIGlet)
  Converted from xbritebi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xbritei.flf (FIGlet)
  Converted from xbritei.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xchartr.flf (FIGlet)
  Converted from xchartr.bdf by bdf2flf (by John Cowan )
  Copyright 1988 Bitstream, Inc., Cambridge, Massachusetts, USA

xchartri.flf (FIGlet)
  Converted from xchartri.bdf by bdf2flf (by John Cowan )
  Copyright 1988 Bitstream, Inc., Cambridge, Massachusetts, USA

xcour.flf (FIGlet)
  Converted from xcour.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xcourb.flf (FIGlet)
  Converted from xcourb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xcourbi.flf (FIGlet)
  Converted from xcourbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xcouri.flf (FIGlet)
  Converted from xcouri.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xhelv.flf (FIGlet)
  Converted from xhelv.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xhelvb.flf (FIGlet)
  Converted from xhelvb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xhelvbi.flf (FIGlet)
  Converted from xhelvbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xhelvi.flf (FIGlet)
  Converted from xhelvi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xsans.flf (FIGlet)
  Converted from xsans.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xsansb.flf (FIGlet)
  Converted from xsansb.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xsansbi.flf (FIGlet)
  Converted from xsansbi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xsansi.flf (FIGlet)
  Converted from xsansi.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xsbook.flf (FIGlet)
  Converted from xsbook.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xsbookb.flf (FIGlet)
  Converted from xsbookb.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xsbookbi.flf (FIGlet)
  Converted from xsbookbi.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xsbooki.flf (FIGlet)
  Converted from xsbooki.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xtimes.flf (FIGlet)
  Converted from xtimes.bdf by bdf2flf (by John Cowan )
  Copyright 1984-1989, 1994 Adobe Systems Incorporated.

xtty.flf (FIGlet)
  Converted from xtty.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered

xttyb.flf (FIGlet)
  Converted from xttyb.bdf by bdf2flf (by John Cowan )
  (c) Copyright Bigelow & Holmes 1986, 1985. Lucida is a registered
