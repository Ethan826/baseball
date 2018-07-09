Retrosheet Event Files

[]{#Top}

![Retrosheet](http://www.retrosheet.org/menubar/retro-logo.gif){.bancenter
width="400" height="46"}

::: {.mbcenter}
-   [Home](http://www.retrosheet.org/)
-   [About ↓](#)
    -   [Overview](http://www.retrosheet.org/site.htm)
    -   [FAQ](http://www.retrosheet.org/faq.htm)
-   [Games/People/Parks ↓](#)
    -   [People →](#)
        -   [Players](http://www.retrosheet.org/boxesetc/index.html#Players)
        -   [Managers](http://www.retrosheet.org/boxesetc/index.html#Managers)
        -   [Coaches](http://www.retrosheet.org/boxesetc/index.html#Coaches)
        -   [Umpires](http://www.retrosheet.org/boxesetc/index.html#Umpires)
        -   [Transactions](http://www.retrosheet.org/transactions/index.html)
    -   [Games →](#)
        -   [Regular
            season](http://www.retrosheet.org/boxesetc/index.html)
        -   [Tiebreaker
            playoffs](http://www.retrosheet.org/Playoff%20Games.htm)
        -   [Post-season](http://www.retrosheet.org/boxesetc/MISC/masterPS.htm)
        -   [All-Star
            games](http://www.retrosheet.org/boxesetc/MISC/masterAS.htm)
    -   [Places →](#)
        -   [Franchises](http://www.retrosheet.org/boxesetc/MISC/FRDIR.htm)
        -   [Ballparks](http://www.retrosheet.org/boxesetc/MISC/PKDIR.htm)
        -   [Birth and
            death](http://www.retrosheet.org/boxesetc/MISC/PNDIR.htm)
    -   [Achievements →](#)
        -   [Awards](http://www.retrosheet.org/boxesetc/MISC/XOH.htm)
        -   [Top
            performances](http://www.retrosheet.org/boxesetc/index.html#TopPerf)
        -   [No-hitters &
            cycles](http://www.retrosheet.org/outstand.htm)
        -   [Milestones](http://www.retrosheet.org/milestones.htm)
-   [Data downloads ↓](#)
    -   [Play-by-play files](http://www.retrosheet.org/game.htm)
    -   [Game logs](http://www.retrosheet.org/gamelogs/index.html)
    -   [Schedules](http://www.retrosheet.org/schedule/index.html)
-   [Features ↓](#)
    -   [Noteworthy events](http://www.retrosheet.org/lists.htm)
    -   [Special features](http://www.retrosheet.org/specfeat.htm)
    -   [Research
        papers](http://www.retrosheet.org/Research/Research.htm)
    -   [Most wanted](http://www.retrosheet.org/wanted/index.html)
-   [Organization ↓](#)
    -   [Who we are](http://www.retrosheet.org/history.htm)
    -   [Discussion group](http://groups.yahoo.com/group/RetroList)
    -   [Donations](http://www.retrosheet.org/donation.htm)
-   [Archives ↓](#)
    -   [Newsletters](http://www.retrosheet.org/news.htm)
    -   [Site history](http://www.retrosheet.org/archives.htm)
:::

\

The Event File

The event files contain game descriptions using the Retrosheet scoring
system. This page will describe the scoring system in sufficient detail
to allow working with these full play-by-play descriptions.

The files containing the play-by-play data follow a naming convention.
Each file has one team's home games and has a name of the form
YYYYTTT.EVX. In this format, YYYY is the four digit year and TTT is a
three character team code. The zip archive downloaded contains a file
named TEAMYYYY that contains the team codes and team names in the
particular season. Each file contains the home games in chronological
order for the specified team.

Files are ASCII text files consisting of a series of records. Each
record is a single line starting with a type designator and ending with
the DOS new line sequence (newline, carriage return characters).

For each game as many as eleven different record types may be used. Each
record type has a unique designator, which is followed by several fields
separated by commas. These are discussed in detail below.

The record type is not considered to be a field and starts in column 1.
Following the record type are the record fields which are separated from
the record type and each other by commas ' , '.

Field data such as names are normally enclosed in double quotes ' " '.
Commas used in quoted fields are not field separators.

[]{#2}Retrosheet player id. All players are represented by a code that
is unique for each player. This 8 character code is constructed from the
first four letters of the player's last name, the first initial of his
common name, and a three digit number. If a player's last name is less
than 4 characters long a dash "-" is used as a placeholder. Numbers
starting with 0 are used for players appearing in games in or after
1983. Players completing their careers before 1983 are assigned numbers
starting with 100.

> joner002 is the Retrosheet player id for Ruppert Jones.

*id* Each game begins with a twelve character ID record which identifies
the date, home team, and number of the game. For example, ATL198304080
should be read as follows. The first three characters identify the home
team (the Braves). The next four are the year (1983). The next two are
the month (April) using the standard numeric notation, 04, followed by
the day (08). The last digit indicates if this is a single game (0),
first game (1) or second game (2) if more than one game is played during
a day, usually a double header The *id* record starts the description of
a game thus ending the description of the preceding game in the file.

>     id,ATL198304080

*version* The version record is next, but is obsolete and can be
ignored.

> version,1

*info* There are up to 34 info records, each of which contains a single
piece of information, such as the temperature, attendance, identity of
each umpire, etc. The record format is info,type,data . The complete
list of [info record types](#1) is given below.

> info,attendance,32737

*start* and *sub* There are 18 (for the NL and pre-DH AL) or 20 (for the
AL with the DH) start records, which identify the starting lineups for
the game. Each start or sub record has five fields. The sub records are
used when a player is replaced during a game. The roster files that
accompany the event files include throwing and batting handedness
information.

1\. The first field is the [Retrosheet player id](#2), which is unique
for each player.

2\. The second field is the player's name.

3\. The next field is either 0 (for visiting team), or 1 (for home team).

4\. The next field is the position in the batting order, 1 - 9. When a
game is played using the DH rule the pitcher is given the batting order
position 0.

5\. The last field is the fielding position. The numbers are in the
standard notation, with designated hitters being identified as position
10. On sub records 11 indicates a pinch hitter and 12 is used for a
pinch runner.

> start,richg001,"Gene Richards",0,1,7

*play* The play records contain the events of the game. Each play record
has 7 fields.

1\. The first field is the inning, an integer starting at 1.

2\. The second field is either 0 (for visiting team) or 1 (for home
team).

3\. The third field is the [Retrosheet player id](#2) of the player at
the plate.

4\. The fourth field is the count on the batter when this particular
event (play) occurred. Most Retrosheet games do not have this
information, and in such cases, "??" appears in this field.

5\. The fifth field is of variable length and contains all pitches to
this batter in this plate appearance and is [described below](#3). If
pitches are unknown, this field is left empty, nothing is between the
commas.

6\. The sixth field describes the play or event that occurred.

> play,5,1,ramir001,00,,S8.3-H;1-2

A play record ending in a number sign, \#, indicates that there is some
uncertainty in the play. Occasionally, a com record may follow providing
additional information. A play record may also contain exclamation
points, "!" indicating an exceptional play and question marks "?"
indicating some uncertainty in the play. These characters can be safely
ignored.

> play,3,1,kearb001,??,,PB.2-3\#\
> com,"Not sure if PB, may have been balk"

The event is the most complex of all the fields and is [described in
detail below](#5).

*badj* This record is used to mark a plate appearance in which the
batter bats from the side that is not expected ("badj" means "batting
adjustment"). The syntax is:

> badj,playerid,hand

The expectation is defined by the roster file. There are two general
cases in which this is used:

1\. Many switch-hitters bat right-handed against right-handed knuckle
ball pitchers even though the default assumption is that they would be
batting left-handed.

> [badj,bonib001,R]{style="font-family: monospace;"}

indicates that switch-hitter Bobby Bonilla was batting right-handed
against a right-handed pitcher.\

2\. Occasionally a player will be listed in a roster as batting "R" or
"L" but will bat the other way. For example, Rick Dempsey did this 13
times in 1983. The syntax this is: badj,dempr101,L

*padj* This record covers the very rare case in which a pitcher pitches
to a batter with the hand opposite the one listed in the roster file. To
date this has only happened once, when Greg Harris of the Expos, a
right-hander, pitched left-handed to two Cincinnati batters on
9-28-1995. The syntax is parallel to that for the badj record:
[padj,harrg001,L]{style="font-family: monospace;"}

*ladj* This record is used when teams bat out of order.

*data* Data records appear after all play records from the game. At
present, the only data type, field 2, that is defined specifies the
number of earned runs allowed by a pitcher. Each such record contains
the pitcher's [Retrosheet player id](#2) and the number of earned runs
he allowed. There is a data record for each pitcher that appeared in the
game.

> data,er,showe001,2

*com* The final record type is used primarily to add explanatory
information for a play. However, it may occur anywhere in a file. The
second field of the com record is quoted.

> com,"ML debut for Behenna"

There is a standard record ordering for each game. An *id* record starts
the description of a particular game. This is followed by the *version*
and *info* records. The *start* records follow the *info* records. The
game is described by a series of *play*, *sub* and *com* records. A
*sub* record is always preceded by a *play np* record. *data* records
follow the last *play* record for the game. A game description is
terminated by an *id* record starting another game or the end of the
file.

### []{#1}Info record types 

Complete records are shown. [info]{style="font-style: italic;"} records
are of two general kinds, game-related and administrative. The order of
these records, which appear after the game id, may not be in the order
shown below. Game-related [info]{style="font-style: italic;"} records
are:

The home and visiting teams are specified by their [Retrosheet team
codes](team_codes.html).

> info,visteam,SDN\
> info,hometeam,ATL

The date is given in conventional yyyy/mm/dd style:

> info,date,1983/04/08

The number record indicates if this is a single game (0), first game (1)
or second game (2) if more than one game is played during a day, usually
this is a double header:

> info,number,0

The hometeam, date and number records duplicate the information in the
*id* record.

Game starting time is given by the two records (0:00 and unknown
indicate missing information):

> info,starttime,7:44PM\
> info,daynight,night

Use of the designated hitter is indicated with true or false:

> info,usedh,false

The presence or absence of pitch information is given. For some games,
the bal-strike counts of the plays are shown, but no pitch detail is
provided. (pitches, count or none):

> [info,pitches,pitches]{style="font-family: monospace;"}

Each umpire and his position on the field are indicated individually by
his Retrosheet ID. For games where umpires are stationed in the
outfield, [umplf]{style="font-family: monospace;"} and
[umprf]{style="font-family: monospace;"} are used. Retrosheet has umpire
assignments for all games in history, except some games in 1979 in which
replacement umpires were used.\

> [info,umphome,quicj901]{style="font-family: monospace;"}\
> [info,ump1b,palld901]{style="font-family: monospace;"}\
> [info,ump2b,engeb901]{style="font-family: monospace;"}\
> [info,ump3b,rungp901]{style="font-family: monospace;"}\
> \

Various field conditions are given:

> info,fieldcond,unknown\
> info,precip,unknown\
> info,sky,night\
> info,temp,69\
> info,winddir,unknown\
> info,windspeed,-1
>
> Values used for fieldcond are: dry, soaked, wet, unknown;\
> for precip: drizzle, none, rain, showers, snow, unknown;\
> for sky: cloudy, dome, night, overcast, sunny, uknown;\
> for winddir: fromcf, fromlf, fromrf, ltor, rtol, tocf, tolf, torf,
> unknown.
>
> Temp(erature) is in degrees Fahrenheit with 0 being the not known
> value.
>
> An unknown windspeed is indicated by -1.

> The BGAME.EXE program outputs these fields using numeric codes:\
> FieldCond: 0 Unknown, 1 Soaked, 2 Wet, 3 Damp, 4 Dry\
> Precip: 0 Unknown, 1 None, 2 Drizzle, 3 Showers, 4 Rain, 5 Snow\
> Sky: 0 Unknown, 1 Sunny, 2 Cloudy, 3 Overcast, 4 Night, 5 Dome\
> WindDir: 0 Unknown, 1 ToLeft, 2 ToCenter, 3 ToRight, 4 LeftToRight, 5
> FromLeft, 6 FromCenter, 7 FromRight, 8 RightToLeft\
> WindSpeed: 0 Unknown, 1 Known, other value is the wind speed

The length of the game in minutes and the attendance (0 used if these
are not known) are given:

> info,timeofgame,134\
> info,attendance,10356

The game site is provided. The site symbols are defined in the file
[parkcode.txt:](http://www.retrosheet.org/parkcode.txt)

> [info,site,SFO02]{style="font-family: monospace;"}

Pitcher win, loss and save data are given as info records. The
Retrosheet player id is used for identification. If no save is credited,
the player id field is empty.

> info,wp,beher001\
> info,lp,sotom001\
> info,save,forst001

When it was used as an official statistic, game winning RBI credit is
given:

> info,gwrbi,chamc001
>
> If this information is unknown or a gwrbi was not credited, the data
> field is left empty.

info records that pertain to how the game account was obtained and
processed (administrative data) are:

> info,edittime,2000/03/31 11:00AM\
> info,howscored,park\
> info,inputprogvers,"version 7RS(19) of 07/07/92"\
> info,inputter,"C. Chestnut"\
> info,inputtime,1995/02/07 9:01PM\
> info,scorer,"Braves"\
> info,translator,"C. Chestnut"

------------------------------------------------------------------------

### []{#3}The pitches field of the play record

synopsis: [play,inning,home/visitor,player
id,count,pitches,event]{style="font-family: monospace;"}

The fifth field, pitches, is a string of variable length and contains
all pitches to this batter in this plate appearance. Most Retrosheet
games do not have pitch data and consequently this field is blank for
such games.

        +  following pickoff throw by the catcher
        *  indicates the following pitch was blocked by the catcher
        .  marker for play not involving the batter
        1  pickoff throw to first
        2  pickoff throw to second
        3  pickoff throw to third
        >  Indicates a runner going on the pitch

        B  ball
        C  called strike
        F  foul
        H  hit batter
        I  intentional ball
        K  strike (unknown type)
        L  foul bunt
        M  missed bunt attempt
        N  no pitch (on balks and interference calls)
        O  foul tip on bunt
        P  pitchout
        Q  swinging on pitchout
        R  foul ball on pitchout
        S  swinging strike
        T  foul tip
        U  unknown or missed pitch
        V  called ball because pitcher went to his mouth
        X  ball put into play by batter
        Y  ball put into play on pitchout

------------------------------------------------------------------------

### []{#5}The event field of the play record

The sixth field, event, describes the play which occurred. This field is
variable in length and has three main portions which define the
Retrosheet scoring system.

The first part of an event is a description of the basic play.

The second part is a modifier for the first part and is separated from
it with a forward slash, "/". In fact, there may be more than one
modifier. A typical use of modifiers is to specify [hit
locations](http://www.retrosheet.org/location.htm). For example, "D8/78"
indicates a double fielded by the center fielder on a ball hit to left
center. A complete list of modifiers excepting hit locations [is given
below](#6). When more than one modifier is used, each is introduced by a
"/".

The third part describes the advance of any runners, separated from the
earlier parts by a period. A successful advance is indicated by a dash,
"-". An out made while advancing is indicated by an X. 2-3 indicates a
runner has advanced from second to third on the play. 1X2 indicates the
runner was out at second advancing from first. If a base runner is not
listed as advancing he remains on the base he was on. In some cases lack
of advance is indicated explicitly by an advance starting and ending on
the same base such as 3-3 . When put outs are made on base runners the
advance field indicates fielding data and errors if they occur. See
below for a [complete description for advances](#4). Note that any
advances after the first are separated by semicolons.

For example, the event "S9/L9S.2-H;1-3" should be read as: single
fielded by the right fielder, line drive to short right field. The
runner on 2nd scored (advanced to home), and the runner on first
advanced to third.

Many event descriptions require information in the form of numbers. The
meaning of a particular number depends on where it appears in the event.
For the descriptions that follow the following notation will be used:

> Fielders will be represented by a number in the range 1 (pitcher) to 9
> (right fielder) using a dollar sign, "\$". When two \$ symbols are
> used, \$\$, this is understood to mean a sequence of two or more
> fielders.
>
> Bases are represented by a percent sign, "%", representing one of five
> characters, 1, 2 and 3 for first through third; B or H for home. B is
> used when a batter advance must be explicitly given. Scoring is
> indicated by an advance that reaches home, H.

Many examples of plays scored using the Retrosheet system will be given
in this document. For some interesting and extreme cases check the
Retrosheet [strange and unusual
plays](http://www.retrosheet.org/strange.htm) listing.

The example plays have been chosen to illustrate how events are coded.
Some of these events are exceedingly rare.

There is occasionally more than one event for each plate appearance,
such as stolen bases, wild pitches, and balks in which the same batter
remains at the plate. On these occasions the pitch sequence is
interrupted by a period, and there is another play record for the
resumption of the batter's plate appearance.

For purposes of description, it is convenient to separate the event
types into two categories: those involving the [batter at the plate](#8)
and [base running](#9) plays that do not involve the batter.

------------------------------------------------------------------------

### []{#8}Events made by the batter at the plate

[\$]{style="font-weight: bold;"} A single fielder represents a fly ball
out made by the specified fielder. Modifiers can be added to indicate
the fly ball trajectory: G for ground ball, L for line drive, P for pop
up, F for a fly ball BG for bunt grounder, BP for bunt pop up. The ball
trajectory code may be followed by a hit location code.

> [play,7,0,saboc001,01,CX,8/F78]{style="font-family: monospace;"}\
> indicates a fly ball caught by the center fielder in left center
> field.

A sacrifice fly is indicated by the modifier SF following a fly out
play. The runner scoring because of the sacrifice is coded in the
advance part of the play.

> play,5,0,grifk001,10,.BX,9/SF.3-H

In the case that a fielder makes an unassisted out on a ground ball a
modifier G follows the event.

> [play,5,0,duncm001,00,X,3/G.2-3]{style="font-family: monospace;"}\
> indicates an unassisted out made by the first baseman with the runner
> on second advancing to third.

[\$\$]{style="font-weight: bold;"} Strings of two or more fielders as an
event specify a ground out where the put out is credited by the last
fielder in the string. Other fielders are credited with assists.

> [play,6,0,davie001,01,FX,63/G6M]{style="font-family: monospace;"}\
> indicates a ground ball out at first on a ball fielded by the
> shortstop.
>
> [play,9,1,pendt001,00,X,143/G1]{style="font-family: monospace;"}\
> More than one player can touch the ball before an out is made. In this
> case, the pitcher has deflected the ball before the second baseman
> threw to first base.
>
> [play,7,1,tempg001,00,X,54(B)/BG25/SH.1-2]{style="font-family: monospace;"}\
> If the putout is made at a base not normally covered by the fielder
> the base runner, batter in this example, is given explicitly.

Force outs are indicated by adding the FO modifier and indicating the
base runner forced.

> [play,5,0,gileb001,10,BX,54(1)/FO/G5.3-H;B-1]{style="font-family: monospace;"}\
> The runner on first is forced at second by a throw from the third
> baseman. The runner on third scores and the batter is safe at first.
> The explicit advance indicated for the batter is optional. A second
> modifier is used to indicate the batted ball trajectory and location.

With the addition of a SH modifier this form is used to indicate
sacrifice hits or bunts that advance a runner.

> play,6,1,camik001,00,X,23/SH.1-2

[\$(%)\$ \$\$(%)\$]{style="font-weight: bold;"} Events of this form are
used to code grounded into double plays.

> [play,7,0,backw001,11,FBX,64(1)3/GDP/G6]{style="font-family: monospace;"}\
> indicates a grounded into double play. The parenthesized 1 indicates
> the base runner on first was the initial out on the play. The GDP
> modifier is followed by a another / and a hit type and location.
>
> [play,8,1,smito001,22,BFCBX,4(1)3/G4/GDP]{style="font-family: monospace;"}\
> An unassisted ground ball out by the second baseman starts this double
> play.

[\$(B)\$(%)]{style="font-weight: bold;"} followed by the modifier LDP is
used to indicate a lined into double play.

> [play,7,0,leonj001,01,CX,8(B)84(2)/LDP/L8]{style="font-family: monospace;"}\
> indicates a fly ball out to the center fielder with the runner on
> second doubled up.
>
> [play,7,0,fernt001,10,BX,3(B)3(1)/LDP]{style="font-family: monospace;"}\
> indicates an unassisted double play by the first baseman who fielded
> the line drive and caught the runner off first base.

The double play notation can be extended in obvious ways to describe
triple plays.

> play,7,1,randw001,00,.\>X,1(B)16(2)63(1)/LTP/L1

[Note:]{style="font-weight: bold;"} the double digit combination 99,
which cannot arise in play, is used to code unknown plays including
forms that otherwise describe force outs and the double plays.
Additional fielders in the double play are assigned 9. No assist or
putout credits are given.\
\
[C/E2 ]{style="font-weight: bold;"}codes catcher interference.
Implicitly, the batter is awarded first unless overridden by an advance
indicating otherwise. A redundant B-1 is allowed.

> play,9,1,cruzj002,??,,C/E2.1-2

[C/E1]{style="font-weight: bold;"} or [C/E3]{style="font-weight: bold;"}
are used when the pitcher or first baseman are called for interfering
with the batter putting him on first without being charged with an at
bat. In these cases C is interpreted as interference by the fielder
specified following the E, not the catcher.

[S\$]{style="font-weight: bold;"} single\
[D\$]{style="font-weight: bold;"} double\
[T\$]{style="font-weight: bold;"} triple\
A hit (excepting a home run) is indicated by one of S, D and T
optionally followed by the fielder, \$, initially handling the ball. If
more than one fielder handles the ball the appropriate sequence of
fielders is given. The fielder designation is omitted if that
information is not known. The batter advance to the designated base is
implicit.

> [play,8,0,pacit001,??,,S7]{style="font-family: monospace;"}\
> is a minimal coding of a single showing that the left fielder first
> handled the ball. The ?? in the count field indicates the count at the
> time of the hit is unknown.
>
> [play,2,1,santn001,12,CFBX,D7/G5.3-H;2-H;1-H]{style="font-family: monospace;"}\
> codes a bases loaded double fielded by the left fielder, a modifier
> showing the hit location code and advances for each of the base
> runners.
>
> [play,3,0,raint001,11,CBX,T9/F9LD.2-H]{style="font-family: monospace;"}\
> describes a triple to right field, a hit location and a runner on
> second scoring.

[DGR]{style="font-weight: bold;"} is the code for a ground rule double.
No fielding player is specified.

> play,3,0,surhb001,10,.BX,DGR/L9LS.2-H

[E\$]{style="font-weight: bold;"} is the code for an error allowing a
batter to get on base. The fielder making the error is given by \$. The
batter advance to first is implicit but may be given explicitly.

> [play,2,0,ruffb001,10,BX,E1/TH/BG15.1-3]{style="font-family: monospace;"}\
> indicates a throwing error (modifier "/TH") error on the pitcher with
> the runner on first advancing to third. The batter advance to first is
> implicit.
>
> [play,5,1,young001,00,X,E3.1-2;B-1]{style="font-family: monospace;"}\
> indicates a fielding error by the first baseman. In this case the
> batter advance to first has been explicitly given.

[FC\$]{style="font-weight: bold;"} Fielder's choice. \$ is the fielder
first fielding the ball. The batter advance to first is understood if it
is not given explicitly.

> [play,4,0,harpb001,22,BBFSFX,FC5/G5.3XH(52)]{style="font-family: monospace;"}\
> The first baseman fielded the ball and threw home in time to retire
> the runner attempting to score. The batter was safe at first.
>
> [play,5,1,jordr001,00,X,FC3/G3S.3-H;1-2]{style="font-family: monospace;"}\
> The first baseman fielded the ball and attempted to throw an
> unspecified runner out. No outs were made and the batter is safe at
> first.\
>
> Note that even though force outs are considered fielder's choices, the
> notation distinguishes between force outs and non-forced fielder's
> choices.\

[FLE\$]{style="font-weight: bold;"} Error on foul fly ball.

> play,5,0,murre001,00,F,FLE5/P5F

[H]{style="font-weight: bold;"} or [HR]{style="font-weight: bold;"} is
the code for a home run leaving the park. The location modifier can be
used to indicate where the ball left the playing field.

> [play,8,0,bellg001,21,CBBX,H/L7D]{style="font-family: monospace;"}\
> indicates a solo home run into left field.
>
> [play,12,1,bichd001,02,FFFX,HR/F78XD.2-H;1-H]{style="font-family: monospace;"}\
> shows a home run into center field with the runners on first and
> second scoring.

[H\$]{style="font-weight: bold;"} or [HR\$]{style="font-weight: bold;"}
indicates an inside-the-park home run by giving a fielder as part of the
code.

> play,4,0,younr001,32,FBFFFBBX,HR9/F9LS.3-H;1-H

[HP]{style="font-weight: bold;"} Batter hit by a pitch. The batter
advance to first is implicit. Other advances are given if needed.

> play,1,1,lansc001,00,H,HP.1-2

[K]{style="font-weight: bold;"} Strike out

> play,1,1,steit001,12,C2FBS,K
>
> [play,6,1,wynnm001,22,..BBFCFS,K23]{style="font-family: monospace;"}\
> A dropped third strike with a putout at first base is given by the
> event K23.

[K+event]{style="font-weight: bold;"} On third strikes various base
running play may also occur. The event can be SB%, CS%, OA, PO%, PB, WP
and E\$.

> [play,2,0,roomr001,12,1BF1S11S,K+PB.1-2]{style="font-family: monospace;"}\
> A passed ball on strike three allowed the runner on first to go to
> second.
>
> [play,5,1,whitd001,02,FLFFS,K+WP.B-1]{style="font-family: monospace;"}\
> An explicit batter advance is given when he reaches first on a third
> strike miscue. An [alternative notation](#7) for WP and PB is given
> below.
>
> [play,8,1,davic001,12,CFB.S,K23+WP.2-3]{style="font-family: monospace;"}\
> Of course, a base running event can occur when the third strike is
> dropped.

[NP]{style="font-weight: bold;"} no play. This event is used as a marker
when substitutions are made.

> play,8,0,puckk001,00,,NP\
> sub,kutcr001,"Randy Kutcher",1,5,8

[I]{style="font-weight: bold;"} or [IW]{style="font-weight: bold;"}
intentional walk\
[W]{style="font-weight: bold;"} walk. In both cases base runner advances
are given if needed. The batter advance to first base is implicit.

> play,6,1,ripkc001,32,CFBBFB\>B,W.1-2
>
> play,8,0,sciom001,30,B+22.III,IW

[W+event]{style="font-weight: bold;"},
[IW+event]{style="font-weight: bold;"}. On ball four various base
running plays may also occur. The event can be SB%, CS%, PO%, PB, WP and
E\$.

> [play,1,1,sandr001,32,C1FBB.BFB,W+WP.2-3]{style="font-family: monospace;"}\
> The fourth ball was a wild pitch allowing the runner on second to
> advance.

------------------------------------------------------------------------

### []{#9}Base-running events not involving the batter

The player specified in these plays is the batter at the plate, not the
base runner or runners affected by the play.

The play pitches and count fields (if given) are for the batter at the
time of the event. Unless the event is a inning or game ending out it
will be followed by another event listing the batter.

[BK]{style="font-weight: bold;"} indicates a balk.

> play,6,0,niekp001,??,,BK.3-H;1-2

[CS%(\$\$)]{style="font-weight: bold;"} is the event code for caught
stealing. The bases, %, for this play are 2,3 and H. The fielding data,
\$\$, is considered part of the play. Other advances may be given.

> play,5,1,ceror001,??,,CSH(12)
>
> play,1,0,bayld001,??,,CS2(24).2-3
>
> [play,6,0,beneb001,??,,CS2(2E4).1-3]{style="font-family: monospace;"}\
> The error negates the out with the advance field indicating a two base
> advance on the play.

[DI]{style="font-weight: bold;"} is the defensive indifference code and
is given when there is no attempt to prevent a stolen base. The advance
field specifies which base the runner went to.

> play,9,0,bencj101,??,,DI.1-2

[OA]{style="font-weight: bold;"} is coded for a base runner advance that
is not covered by one of the other codes. A comment may be given
explaining the advance.

> play,3,1,parkr001,??,,OA.2X3(25)\
> com,"Thompson out trying to advance after ball eluded catcher"

[PB]{style="font-weight: bold;"} passed ball\
[WP]{style="font-weight: bold;"} wild pitch. In both cases the catcher
is unable to handle a pitch and a base runner advances.

> play,1,1,jackb001,12,FBSFFB,WP.2-3;1-2
>
> play,1,1,evand002,01,CB,PB.2-3

[PO%(\$\$)]{style="font-weight: bold;"} picked off of base % (1, 2 or 3)
with the (\$\$) indicating the throw(s) and fielder making the putout.

> [play,4,0,guerp001,00,22,PO2(14)]{style="font-family: monospace;"}\
> indicates the runner on second was out by a pick off throw from the
> pitcher to second baseman.
>
> [play,1,1,wallt001,10,B11,PO1(E3).1-2]{style="font-family: monospace;"}\
> shows an attempt at a pick off at first with the first baseman
> committing an error that allows the runner to advance to second. The
> presence of the error (E3) negates the out normally associated with
> the pickoff play.

[POCS%(\$\$)]{style="font-weight: bold;"} picked off off base % (2, 3 or
H) with the runner charged with a caught stealing. The (\$\$) is the
sequence of throws resulting in the out.

> play,6,1,javis001,10,B1,POCS2(1361)

[SB%]{style="font-weight: bold;"} is the event code for a stolen base.
The bases, %, for this play are 2,3 and H.

> play,6,0,benzt001,11,BSB,SB2
>
> [play,4,1,waltj001,10,BB,SB3;SB2]{style="font-family: monospace;"}\
> [play,4,1,shefg001,12,SP1CB,SBH;SB2]{style="font-family: monospace;"}\
> show double steals, second and third in one case, second and home in
> the other.

------------------------------------------------------------------------

### []{#6}Play modifiers and explanations

Each modifier is preceded by / in a play record. As always, % indicates
one the four bases and \$ indicates a fielder.

    AP    appeal play
    BP    pop up bunt
    BG    ground ball bunt
    BGDP  bunt grounded into double play
    BINT  batter interference
    BL    line drive bunt
    BOOT  batting out of turn
    BP    bunt pop up
    BPDP  bunt popped into double play
    BR    runner hit by batted ball
    C     called third strike
    COUB  courtesy batter
    COUF  courtesy fielder
    COUR  courtesy runner
    DP    unspecified double play
    E$    error on $
    F     fly
    FDP   fly ball double play
    FINT  fan interference
    FL    foul
    FO    force out
    G     ground ball
    GDP   ground ball double play
    GTP   ground ball triple play
    IF    infield fly rule
    INT   interference
    IPHR  inside the park home run
    L     line drive
    LDP   lined into double play
    LTP   lined into triple play
    MREV  manager challenge of call on the field
    NDP   no double play credited for this play
    OBS   obstruction (fielder obstructing a runner)
    P     pop fly
    PASS  a runner passed another runner and was called out
    R$    relay throw from the initial fielder to $ with no out made
    RINT  runner interference
    SF    sacrifice fly
    SH    sacrifice hit (bunt)
    TH    throw
    TH%   throw to base %
    TP    unspecified triple play
    UINT  umpire interference
    UREV  umpire review of call on the field



    Event advances.
    In addition to base runner movements, the advance
    portion of an event indicates fielding, errors and has the indicators
    indicating if a run is unearned and if an RBI is or is not
    credited.
    Bases are represented by one of five characters, 1
    for first, 2, 3 and B or H for home. B is used when a batter advance
    must be explicitly given. Scoring is indicated by a successful
    advance that reaches home, H.
    Separate advances are given for each runner on
    base and are separated by a semicolon, ";". When more than one runner
    advance is given for a play they are ordered starting with the runner
    on third base and ending with the batter.
    Advances may include additional information in the
    form of one or more parameters specified as a parenthesized strings
    of characters. When more than one parameter is given on an advance
    they are individually parenthesized.
    A successful advance is given in the form 1-2. The
    dash "-" indicates a successful advance. Multiple base advances are
    indicated with the same notation: B-2, 1-3, 1-H, 2-H. 
    play,1,1,marte001,32,BBCBFFB,W.2-3;1-2
      play,3,1,stilk001,11,CBX,S7/F7S.2-H;B-2

    A runner put out at a particular base is indicated
    by the "X": 2X3, 1XH. When a runner is out the advance gives the
    fielding information as a parameter specifying the fielders. The last
    fielder gets credit for the put out and the others get
    assists.
    play,4,1,stubf001,32,CBFBBFFS,K/DP.1X2(26)
      play,6,0,murre001,22,BSFFBX,9/F9LS/FDP.3XH(92)
      play,4,0,blauj001,01,CX,S8/L78.BX2(8434)

    Fielding errors are indicated by including an E in
    the parameter following an advance. The fielder following the E is
    charged with the error.
    play,3,0,fielc001,00,X,S7/L7LD.3-H;2-H;BX2(7E4)

    Following a second baseman error the batter is safe at second. The
    error indicator negates the out. The left fiellder is credited with an
    assist.
      play,7,0,puckk001,01,CX,S5/G5.1-3(E5/TH)

    The parameter in this play attributes a throwing error to the third
    baseman. A base indicator may follow TH, TH2 for example.

    Parameters are used to indicate if a run is
    unearned (UR) and if RBI is to be credited (RBI) or not (NR),
    (NORBI). When these parameters are not present, normal rules are
    followed.
    play,9,0,davie001,30,BBBB,W+PB.3-H(NR);1-3

    The run scored on the passed ball is not credited as an RBI to the
    batter.
      play,8,1,sax-s001,22,BCFBFX,S4/G34.2-H(E4/TH)(UR)(NR);1-3;B-2

    Three parameters are given on the 2-H advance. The first indicates a
    second baseman throwing error, the second indicates it is an unearned
    run and the third indicates no RBI.
      play,2,1,willk001,11,BFX,E6/G6.3-H(RBI);2-3;B-1

    In this play an RBI is given to the batter.

    Interference can be indicated with an advance
    parameter. An alternative way of writing this is (5/INT).
    play,2,0,stanp001,12,CCBX,S/L9S.3-H;2X3(5/INT);1-2

    com,"$Gonzalez out for grabbing coach on way back to 3B"
    Team unearned runs are indicated by TUR in cases
    with more than one picther in the inning and the current pitcher is to
    be
    charged with an earned run.
    play,5,1,ashba001,??,,S9.3-H(TUR);2-H(TUR);1-3;BX2(93)
    A U appearing in a fielding sequence indicates
    the fielder handling the ball is unknown.
    play,7,0,perrg001,21,B.BFX,S8.2-H;BX2(8U3)

    In the 8U3 sequence most likely the U is the shortstop or second
    baseman.
    Advance parameters provide an
    alternative way of indicating wild pitches and passed
    balls.
    play,5,0,feldm001,22,1LPB>F1S,K.1-2(WP)

    ladj. This record is used when teams bat out of order. The normal
    assumption is that proper lineup sequence is followed, therefore, it
    is necessary to have some special indication when this is violated.
    The format is:
    ladj,hv,pos
    where "hv" is 0 for visiting or 1 for the home
    team and "pos" is 1-9 for the batting order position. Retrosheet has
    discovered quite a few cases of batting out of turn. You can see them
    in the Special Lists section: Batting Out of Turn.
    Here are some examples.

    play,2,1,hortw101,??,,63
    ladj,1,7
    play,2,1,simpj101,??,,D7/BOOT
    ladj,1,6
    play,2,1,steib001,??,,HP/BOOT
    ladj,1,8
    play,2,1,cox-l101,??,,S9/BOOT.2-3;1-2
    play,2,1,mendm101,??,,NP
    sub,robel001,"Leon Roberts",1,9,11
    play,2,1,robel001,??,,64(1)3/GDP


    play,5,1,talbf101,??,,NP
    sub,rollr101,"Rich Rollins",1,9,11
    play,5,1,rollr101,??,,S8
    play,5,1,harpt101,??,,S/B.1-2
    ladj,1,4
    play,5,1,simpd102,??,,K/BOOT
    ladj,1,5
    play,5,1,comew101,??,,8/BOOT
    ladj,1,4
    play,5,1,simpd102,??,,2/BOOT
    com,"$Davis is called out for batting out of order;"
    com,"he doubled in 2 runs which triggered the protest;"
    com,"since Simpson was the one due up, he was charged with the out"

    Note that every batting out of turn situation has
    its own character, including whether or not it is detected by the
    opposition and whether or not the incorrect batter makes an out or
    reaches safely.
    play,5,0,feldm001,22,1LPB>F1S,K.1-2(WP)

    Replay

    Instant replay of home run calls was instituted on 8/28/2008. It was expanded at the start of the 2014 
    season to include many other types of plays. For a more complete explanation and list all replays, see 
    the following two pages.

    http://www.retrosheet.org/ReplayHR.htm
    http://www.retrosheet.org/Replay.htm

    Each time the replay system is used, a slash tag is added to the play string. This will be /UREV for an 
    umpire-initiated review and /MREV for a manager challenge. Immediately after that play there will be a 
    comment record with details of the replay/challenge. The fields in this string are:

    com,"replay,inning,Batter ID,Batter Team ID,Umpire ID,Ballpark ID,Reason,Reversed,Initiator,Team,Type 
    Code"

       Inning: inning in which the replay occurred
       Batter ID: batter for the replay instance (not necessarily the player involved in the replay)
       Batter Team ID: the team at bat for the replay
       Umpire ID: crew chief
