# Retrosheet

We can base a lot of this off the prior art of Retrosheet.

Here is an example of a portion of a roster file:

    almoa002,Almora,Albert,R,R,CHN,OF
    andeb004,Anderson,Brett,L,L,CHN,P
    arrij001,Arrieta,Jake,R,R,CHN,P
    avila001,Avila,Alex,L,R,CHN,C
    baezj001,Baez,Javier,R,R,CHN,2B
    bryak001,Bryant,Kris,R,R,CHN,3B
    butle001,Butler,Eddie,R,R,CHN,P
    candj002,Candelario,Jeimer,B,R,CHN,3B

Here is an example of a portion of a team file:

    ARI,N,Arizona,Diamondbacks
    ATL,N,Atlanta,Braves
    CHN,N,Chicago,Cubs
    CIN,N,Cincinnati,Reds

Here is an example of a portion of an event file:

    id,CHN201704100
    version,2
    info,visteam,LAN
    info,hometeam,CHN
    info,site,CHI11
    info,date,2017/04/10
    info,number,0
    info,starttime,9:01PM
    info,daynight,night
    info,usedh,false
    info,umphome,holbs901
    info,ump1b,gibsg901
    info,ump2b,nauep901
    info,ump3b,iassd901
    info,howscored,park
    info,pitches,pitches
    info,oscorer,frisd701
    info,temp,41
    info,winddir,fromcf
    info,windspeed,8
    info,fieldcond,unknown
    info,precip,unknown
    info,sky,unknown
    info,timeofgame,217
    info,attendance,41166
    info,wp,daviw001
    info,lp,romos001
    info,save,
    start,forsl001,"Logan Forsythe",0,1,4
    start,seagc001,"Corey Seager",0,2,6
    start,turnj001,"Justin Turner",0,3,5
    start,gutif001,"Franklin Gutierrez",0,4,7
    start,puigy001,"Yasiel Puig",0,5,9
    start,gonza003,"Adrian Gonzalez",0,6,3
    start,grany001,"Yasmani Grandal",0,7,2
    start,pedej001,"Joc Pederson",0,8,8
    start,wooda002,"Alex Wood",0,9,1
    start,schwk001,"Kyle Schwarber",1,1,7
    start,bryak001,"Kris Bryant",1,2,5
    start,rizza001,"Anthony Rizzo",1,3,3
    start,zobrb001,"Ben Zobrist",1,4,9
    start,russa002,"Addison Russell",1,5,6
    start,contw001,"Willson Contreras",1,6,2
    start,heywj001,"Jason Heyward",1,7,8
    start,baezj001,"Javier Baez",1,8,4
    start,lestj001,"Jon Lester",1,9,1
    play,1,0,forsl001,22,BCCFBC,K
    play,1,0,seagc001,32,FFBBBX,3/G
    play,1,0,turnj001,21,BCBX,9/F
    play,1,1,schwk001,01,SX,7/F
    play,1,1,bryak001,31,BBSBB,W
    play,1,1,rizza001,00,X,9/F+
    play,1,1,zobrb001,02,CFC,K
    play,2,0,gutif001,31,CBBBB,W
    play,2,0,puigy001,32,BFFB+1B>S,K+CS2(24)/DP
    play,2,0,gonza003,12,CBCX,8/L
    play,2,1,russa002,00,,NP
    sub,vanss001,"Scott Van Slyke",0,4,7
    com,"Dodgers left fielder Franklin Gutierrez left the game due to an injured leg"

    ...

    data,er,wooda002,1
    data,er,strir001,0
    data,er,fielj002,0
    data,er,daytg001,0
    data,er,romos001,1
    data,er,jansk001,0
    data,er,lestj001,1
    data,er,edwac001,0
    data,er,grimj002,0
    data,er,strop001,0
    data,er,uehak001,0
    data,er,daviw001,0

## Event file fields

The following is copied from http://www.retrosheet.org/eventfile.htm1

### `id`

> Each game begins with a twelve character ID record which identifies
> the date, home team, and number of the game. For example, ATL198304080
> should be read as follows. The first three characters identify the
> home team (the Braves). The next four are the year (1983). The next
> two are the month (April) using the standard numeric notation, 04,
> followed by the day (08). The last digit indicates if this is a single
> game (0), first game (1) or second game (2) if more than one game is
> played during a day, usually a double-header.

    id,ATL198304080

### `version`

> The version record is next, but is obsolete and can be ignored.

    version,1

### `info`

> There are up to 34 info records, each of which contains a single piece
> of information, such as the temperature, attendance, identity of each
> umpire, etc. The record format is `info,type,data`.

    info,attendance,32737

### `start` and `sub`

start and sub There are 18 (for the NL and pre-DH AL) or 20 (for the AL
with the DH) start records, which identify the starting lineups for the
game. Each start or sub record has five fields. The sub records are used
when a player is replaced during a game. The roster files that accompany
the event files include throwing and batting handedness information.

1.  The first field is the Retrosheet player id, which is unique for
    each player.
2.  The second field is the player’s name.
3.  The next field is either 0 (for visiting team), or 1 (for home
    team).
4.  The next field is the position in the batting order, 1 - 9. When a
    game is played using the DH rule the pitcher is given the batting
    order position 0.
5.  The last field is the fielding position. The numbers are in the
    standard notation, with designated hitters being identified as
    position 10. On sub records 11 indicates a pinch hitter and 12 is
    used for a pinch runner.

    start,richg001,“Gene Richards”,0,1,7

### `play`

The `play` records contain the events of the game. Each play record has 7
fields.

1.  The first field is the inning, an integer starting at 1.
2.  The second field is either 0 (for visiting team) or 1 (for home
    team).
3.  The third field is the Retrosheet player id of the player at the
    plate.
4.  The fourth field is the count on the batter when this particular
    event (play) occurred. Most Retrosheet games do not have this
    information, and in such cases, “??” appears in this field.
5.  The fifth field is of variable length and contains all pitches to
    this batter in this plate appearance and is described below. If
    pitches are unknown, this field is left empty, nothing is between
    the commas.
6.  The sixth field describes the play or event that occurred.

    play,5,1,ramir001,00,,S8.3-H;1-2

A play record ending in a number sign, \#, indicates that there is some
uncertainty in the play. Occasionally, a `com` record may follow
providing additional information. A play record may also contain
exclamation points, “!” indicating an exceptional play and question
marks “?” indicating some uncertainty in the play. These characters can
be safely ignored.

    play,3,1,kearb001,??,,PB.2-3#
    com,"Not sure if PB, may have been balk"

### `badj`

This record is used to mark a plate appearance in which the batter bats
from the side that is not expected (“badj” means “batting adjustment”).
The syntax is:

    badj,playerid,hand

The expectation is defined by the roster file. There are two general
cases in which this is used:

1.  Many switch-hitters bat right-handed against right-handed knuckle
    ball pitchers even though the default assumption is that they would
    be batting left-handed.

    badj,bonib001,R

indicates that switch-hitter Bobby Bonilla was batting right-handed
against a right-handed pitcher.

2.  Occasionally a player will be listed in a roster as batting “R” or
    “L” but will bat the other way. For example, Rick Dempsey did this
    13 times in 1983. The syntax this is: badj,dempr101,L

padj This record covers the very rare case in which a pitcher pitches to
a batter with the hand opposite the one listed in the roster file. To
date this has only happened once, when Greg Harris of the Expos, a
right-hander, pitched left-handed to two Cincinnati batters on
9-28-1995. The syntax is parallel to that for the badj record:
padj,harrg001,L

### `ladj`

This record is used when teams bat out of order.

### `data`

Data records appear after all play records from the game. At present,
the only data type, field 2, that is defined specifies the number of
earned runs allowed by a pitcher. Each such record contains the
pitcher’s Retrosheet player id and the number of earned runs he allowed.
There is a data record for each pitcher that appeared in the game.

    data,er,showe001,2

### `com`

The final record type is used primarily to add explanatory information
for a play. However, it may occur anywhere in a file. The second field
of the com record is quoted.

    com,"ML debut for Behenna"

There is a standard record ordering for each game. An `id` record starts
the description of a particular game. This is followed by the `version`
and `info` records. The `start` records follow the `info` records. The
game is described by a series of `play`, `sub` and `com` records. A
`sub` record is always preceded by a `play` np record. `data` records
follow the last play record for the game. A `game` description is
terminated by an `id` record starting another game or the end of the
file.

## Some observations

Retrosheet is event-driven. Each event is prefixed by one of `play`
