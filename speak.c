#
/*	Voice Synthesizer Program "Speak"			*/
/*	Copyright 1974, Bell Telephone Laboratories, Inc.	*/
/*								*/
/*	Language: C						*/
/*	Programmer: M. D. McIlroy				*/
/*								*/
/*	For description of method see "Synthetic English	*/
/*	Speech by Rule", M. D. McIlroy, Bell Telephone		*/
/*	Laboratories, Inc. 1974					*/
/*								*/
#define NT 800
#define NS 9500

#ifdef __unix__
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#endif

#ifdef __STDC__
#include <stdlib.h>
#define P(args)  args
#else
#define P(args)  ()
#define void
#endif

struct decenc {
	int x,y;
};

int compare P((char *a,char *b));
void copy P((void));
void decode P((int f, char *s));
int dencode P((int t1[],int t2[], int c));
void diag P((int n));
void finale P((char *in,char **s));
char finals P((char *in,char **ls));
int find P((char *in));
int fold P(( char *s));
void insert P((char *in,char **ls));
void list P((int f, char *s));
char *longe P((char *in,char *end));
void mide P((char *in,char **ls));
void mids P((char *in,char *end));
void midu P((char *in,char *end));
char *name P((void));
int oneof P((char c, char *l));
char *phpron P((char *in,char *out));
char *phread P((char *in,char *out));
char *phspell P((char *in,char *out));
void phwrite P((char *in,char *out));
int prefix P((char *in));
void readin P((char *file));
int replace P((void));
void stash P((char *s));
char* suffix P((char *in,char *end,char **s));
int syltest P((char *in, char *s, char *end));
int th P((char *s));
char *vowel P((char *in,char *end));
void writeout P((char * file));
int writo1 P((int f,int *u,int n));


char *diags[] = {
	0,
	"bad option",
	"no vocabulary",
	"can't create",
	"too many words",
	"too many chars" };

char *aeiou = "aeiou";
char *aeiouy = "aeiouy";
char *aeiouwxy = "aeiouwxy";
char *aeo = "aeo";
char *aou = "aou";
char *bcdfgkpt = "bcdfgkpt";

struct rec {
	int word,phon;
};
int recsize = sizeof(struct rec);
struct rec table[NT];
char strings[NS];
int ttop = 1;
int stop = 1;
char buf[100];
int eflag;
int tflag = 0;

#define p(a, b) ((b << 8) | a)

int code[] = {
	p('a','0'),	033,	/*AH--co_ntact, ca_r*/
	p('a','1'),	052,	/*AH1--co_nnect*/
	p('a','2'),	067,	/*AH2--*/
	p('W','0'),	002,	/*AW--ca_ll, law__ (,l,u2,aw)*/
	p('W','1'),	054,	/*AW1--fau__lt*/
	p('W','2'),	017,	/*AW2--*/
	p('a','e'),	021,	/*AE--ca_t, sa_t*/
	p('e','a'),	020,	/*EA1--a_ntenna*/
	p('A','0'),	037,	/*A--na_me (,n,ai,y0,m)*/
	p('A','1'),	071,	/*A1--na_mely*/
	p('A','2'),	072,	/*A2--*/
	p('e','0'),	004,	/*EH--me_t, e_nter*/
	p('e','1'),	075,	/*EH1--*/
	p('e','2'),	076,	/*EH2--*/
	p('e','3'),	077,	/*EH3--*/
	p('e','r'),	005,	/*ER--weather__*/
	p('E','0'),	023,	/*E--three__*/
	p('E','1'),	026,	/*Y--sixty_*/
	p('E','2'),	035,	/*Y1--y_es*/
	p('y','0'),	003,	/*IE--ze_ro*/
	p('y','1'),	036,	/*AY--may_*/
	p('i','0'),	030,	/*I--si_x*/
	p('i','1'),	064,	/*I1--i_nept, i_nside*/
	p('i','2'),	065,	/*I2--stati_c*/
	p('i','3'),	066,	/*I3--*/
	p('o','0'),	031,	/*O--o_nly, no_*/
	p('o','1'),	012,	/*O1--hello_*/
	p('o','2'),	013,	/*O2--no_tice*/
	p('o','u'),	051,	/*OO1--goo__d shou__ld*/
	p('e','u'),	011,	/*IU--new_*/
	p('o','o'),	050,	/*OO--loo__k*/
	p('u','0'),	014,	/*UH--bu_t*/
	p('u','1'),	015,	/*UH1--u_ncle*/
	p('u','2'),	016,	/*UH2--stirru_p*/
	p('u','3'),	034,	/*UH3--app_le ab_le*/
	p('U','0'),	027,	/*U--u_se*/
	p('U','1'),	010,	/*U1--u_nite*/
	'b',		061,
	'd',		041,
	p('d','t'),	073,	/*OOH--Goe__the cheveu__x(?)*/
	'f',		042,
	'g',		043,
	'h',		044,
	'k',		046,
	'l',		047,
	'm',		063,
	'n',		062,
	'p',		032,
	'r',		024,
	's',		040,
	't',		025,
	'v',		060,
	'w',		022,
	'z',		055,
	p('s','h'),	056,	/*SH--sh__ow, sh__ip*/
	p('z','h'),	070,	/*ZH--pleas_ure*/
	'j',		045,	/*J--edge_ */
	p('c','h'),	057,	/*CH--batch__*/
	p('t','h'),	006,	/*TH--th__in*/
	p('d','h'),	007,	/*THV--th__en*/
	p('n','g'),	053,	/*NG--long__, in_k*/
	p('-','0'),	001,	/*PA1*/
	p('-','1'),	074,	/*PA0--short pause*/
	0,		0
};
char work[100];
char line[100];
/* Voice synthesizer file descriptor */
int vs = 1;

int
main(argc,argv)
int argc;
char **argv;
{
	char register *t,*u;
	int i;
	char *wtop;
	int f;
	int pflag,sflag,vflag,lflag;
	int xflag,yflag,wflag;
	char register *w;
	char *v;
	pflag =  eflag =sflag = vflag = 1;

	if(argc>1 && *argv[1]=='-') {
loop:	
		switch(*(argv[1]++)) {
		default:
			goto loop;
		case 'e':
			eflag = 0;
			goto loop;
		case 'p':
			pflag = 0;
			goto loop;
		case 's':
			sflag = 0;
			goto loop;
		case 'v':
			vflag = 0;
			goto loop;
		case '\0':;
		}
		argc--;
		argv++;
	}
	if(vflag) vs = argc>2?creat(argv[2],0666):1;
	readin(argc>1 ? argv[1]:"/usr/lib/speak.m");
	for(;;)	{

		t = line;
		do	{
			if(!read(0,t,1)) {
				exit(0);
			}
		}
		while(*t++!='\n');
		*(t-1)=' ';
		*t=0;
		if(line[0]=='!') switch(line[1]) {
		case 'c':
			copy();
			break;
		case 'w':
			writeout(name());
		case 'r':
			t = name();
			if(*t) readin(t);
			break;
		case 'd':
			if(phread(work,buf+1)!=buf+1)
				decode(1,buf+1);
			else {
				tflag = 1;
				phpron(work,buf);
				tflag = 0;
			}
			write(1,"\n",1);
			break;
		case 'p':
			decode(1,buf+1);
			write(1,"\n",1);
			break;
		case 'l':
			i = 0;
			while(++i<ttop)
				list(1,&strings[table[i].word]);
			break;
		default:
			diag(1);
			break;
		}
		else	{
			if(!replace()&&line[1]!='\0') {
				t = line;
				u = work;
				lflag = 0;
				for(;;) {
					while(*t==' '||*t=='\t') t++;
					while((*u= *t++)!=' '&&*u!='\t') {
						if(*u) {
							if('a'<=*u&&'z'>=*u
							    ||*u=='%')
								lflag++;
							u++;
						}
						else goto next;
					}
					*u++ = 0;
				}
next:
				wtop = u;
			}
			t = work;
			while(t<wtop){
				u = phread(t,buf+1);
				wflag=yflag=0;
				if(u==buf+1 && pflag) {
					for(v=t;oneof(*v,"([`\"");v++);
					for(w=v;*w;w++)
						if(*w=='-'&&w-v>=2){
							wflag = *w;
							*w = 0;
							break;
						}
					if(!wflag){
						for(;oneof(*--w,".,;:?!'\"])"););
						yflag = *++w;
					}
					if(w<=v) goto noword;
					*w = 0;
					xflag = 0;
					if(!lflag) for(u=v;*u;u++)
						if(fold(u)) xflag++;
					if((yflag||xflag||wflag)&&
					    (u=phread(v,buf+1))!=buf+1);
					else u = phpron(v,buf+1);
					if((*w=yflag)&&u!=buf+1) {
						/*pause for punct*/
						*u++ = 001;
						*u++ = 001;
					}
				}
noword:
				if(u==buf+1&&wflag)
					*w = wflag;
				if(u==buf+1&&sflag)
					u = phspell(t,buf+1);
				*buf = 0174;	/*phoneme ,2-1*/
				*u++ = 0174;	/* temp */
				*u++ = 0;
				if(vflag) write(vs,buf,u-buf);
				while(*t++);
			}
		}
	}
}

/* Decode s for listing */
void
decode(f,s)
int f;
char *s;
{
	int b,c;
	int register flag;
	flag = 1;
	while(c = *s++) {
		if(flag) {
			write(f,",",1);
			b ='3'- ((c&0377)>>6);
			if(c==001) {
				flag = 0;
				c = '%';
			}
			else {
				if(b!='2') write(1,&b,1);
				c = c&077;
				c = dencode(&code[1],&code[0],c);
			}
		}
		while(c){
			write(f,&c,1);
			c>>=8;
		}
	}
}

/*
 * Encode and decode from the first column of code to the second, and
 * the opposite.
 * Examples:
 * assert(dencode(&code[0],&code[1],'u3') == 034);
 * assert(dencode(&code[1],&code[0],034) == 'u3');
 */
int
dencode(at1,at2,c)
int at1[], at2[], c;
{
	/*
	 * Through this cast search and results iterate over the corresponding
	 * table column.
	 */
	struct decenc {
	       int x,y;
	};
	struct decenc *t1 = (struct decenc *)at1, *t2 = (struct decenc *)at2;
	int register i,d;
	for(i=0;d=t1[i].x;i++) if(c==d) break;
	return(t2[i].x);
}

int
replace(){
	char register *t,*u;
	int register n;
	int b,i;
	t = line;
	u = buf;
	if(*t++!=',')return(0);
	for(;;) {
		if(*t=='%') {
			*u++ = 001;
			while((*u = *++t) && *u!=' ') u++;
			break;
		}
		b = 1;
		if(*t<='3') if(*t>='0')
			b = '3'-*t++;
		n = 0;
		while(*t!=',' && *t!=' ' && *t!=0) {
			i = *t++;
			if(n) i<<=8;
			n |= i;
		}
		n = dencode(&code[0],&code[1],n);
		if(n) *u++ = n+ (b<<6);
		if(*t!=',' && *t!=' ') break;
		t++;
	}
	*u++=0;
	phwrite(work,buf);
	return(1);
}

void
list(f,s)
int f;
char *s;
{
	char register *t;
	if(phread(s,buf)==buf) return;
	write(f," ",1);
	t = s;
	while(*t) write(f,t++,1);
	write(f,"\n",1);
	decode(f,buf);
	write(f,"\n",1);
}

void
copy(){
	char buf1[100];
	phread(work,buf1);
	phwrite(name(),buf1);
}

char *
name(){
	char register *u,*t;
	u = &line[2];
	while(*u==' ') u++;
	t = buf;
	while(*u && (*t = *u++)!=' ') t++;
	*t = 0;
	return(buf);
}

void
readin(file)
char *file;
{
	int register f;
	if((f = open(file,0))<0) {
		diag(2);
		return;
	}
	read(f,&ttop,sizeof(ttop));
	read(f,table,recsize*ttop);
	read(f,&stop,sizeof(stop));
	read(f,strings,stop);
	close(f);
}

int
writo1(f,u,n)
int f, n;
int *u;
{
	int register i,j,k;
	i = j = *u;
	*u = n;
	k = 1;
	while(strings[i++]) k++;
	write(f,&strings[j],k);
	return(k);
}

void
writeout(file)
char *file;
{
	int register f,i;
	int n;
	if((f=creat(file,0666))<0) {
		diag(3);
		return;
	}
	lseek(f,recsize*ttop+sizeof(ttop)+sizeof(n),0); /*get to byte 0 of string store*/
	write(f,strings,1);	/*and put it out*/
	n = 1;
	for(i=1;i<ttop;i++) {
		n += writo1(f,&table[i].word,n);
		n += writo1(f,&table[i].phon,n);
	}
	lseek(f,0,0);
	write(f,&ttop,sizeof(ttop));
	write(f,table,recsize*ttop);
	write(f,&n,sizeof(n));	/*new value of stop */
	close(f);
}


int
find(in)
char *in;
{
	int register bot,top,i;
	int z;
	bot = 0;
	top = ttop;
	z = 0;
	while((i=(bot+top)/2)>bot) {
		z = compare(in,&strings[table[i].word]);
		if(z==0) break;
		if(z<0) top = i;
		else bot = i;
	}
	return(i);
}

int
prefix(in)
char *in;
{
	char register *u,*s;
	char *end;
	int register i;
	int pref,bot,top;
	pref = bot = 0;
	top = ttop;
	end = in+1;	/* +1 saves wasted time looking up % */
loop:	
	while((i=(bot+top)/2)>bot) {
		do {
			s = &strings[table[i].word];
			for(u=in;;u++) {
				if(*u<*s) top = i;
				else if(*u== *s++) {
					if(*u==0) return(i);
					if(*s!=0) continue;
					pref = bot = i;
					end = u+1;
				}
				else if(u<=end) bot = i;
				else break;
				goto loop;
			}
		}
		while((i=(bot+i)/2)>bot);
		bot++;
		end++;
	}
	return(pref);
}

int
compare(a,b)
char *a,*b;
{
	while(*a == *b) {
		if(*a==0) return(0);
		a++;
		b++;
	}
	return(*a<*b ? -1 : 1);
}

char *
phread(in,out)	/* returns address of letter after output string */
char *in,*out;
{
	char *s;
	int i;
	i = find(in);
	if(compare(in,&strings[table[i].word])==0) {
		for(s = &strings[table[i].phon];*out = *s++;)
			out++;
	}
	*out = 0;
	return(out);
}

void
phwrite(in,out)
char *in, *out;
{
	int register i,j,z;
	i = find(in);
	if(0!=(z=compare(in,&strings[table[i].word]))) {
		if(*out==0) return;
		i++;
		if(ttop>=NT) {
			diag(4);
			return;
		}
		for(j=ttop;j>i;j--) {
			table[j].word = table[j-1].word;
			table[j].phon = table[j-1].phon;
		}
		table[i].word = stop;
		stash(in);
		ttop++;
	}
	else if(*out==0) {
		for(j=i;j<ttop;j++) {
			table[j].word = table[j+1].word;
			table[j].phon = table[j+1].phon;
		}
		ttop--;
		return;
	}
	table[i].phon = stop;
	stash(out);
}

/* Add the passed string to the string table, updating stop */
void
stash(s)
char *s;
{
	while(stop<NS)
		if((strings[stop++]= *s++)==0)
			return;
	diag(5);
}

void
diag(n)
int n;
{
	char register *p;
	p = diags[n];
	while(*p) write(1,p++,1);
	write(1,"\n",1);
}

char *
phspell(in,out)
char *in, *out;
{
	char register *t;
	char c[] = "* \0";
	while(c[1] = *in++) {
		fold(&c[1]);
		t = phread(c,out);
		if(t!=out) out = t;
		else {
			*out++ = 0346;	/* ,0k */
			*out++ = 0367;	/* ,0a2 */
		}
		if(*in) *out++ = 0101;
	}	/*phoneme 2-0*/
	*out = 0;
	return(out);
}


/*danger--reuses "line" to conserve space*/
char *
phpron(in,out)
char *in, *out;
{
	char register *t,*u;
	char *s;
	char register *sout;
	char sflag;
	int i;
	sout = out;
	*sout = 0;
	s = t = line+2;
	u = in;
	while(*s++ = *u++);
	s -= 2;
	sflag = 0;
	if(fold(t)) if(sout!=(out=phread(t,out)))
		return(out);
	if(s==t||vowel(t,s+1)<t) {
		/*spell one-letter words and vowelless words*/
		goto done;
	}
	if(eflag) {	/* handle english endings*/
		if(sflag = finals(t,&s))
			if(sout!=(out=phread(t,out))) {
				*out++ = sflag>1?0140:0155;
				*out++ = 0140; /*,s,s or ,z,s*/
				goto done;
			}
		midu(t,s);
		finale(t,&s);
		mide(t,&s);
		mids(t,s);
		if(sflag) *++s = 's';
	}
	*--t = '#';
	*++s = '#';
	*++s = 0;
	while(*t) {
		*--t = '%';
		i = prefix(t);
		if(i==0) {
			*sout = 0;
			return(sout);
		}
		u = &strings[table[i].word];
		while(*u) {
			t++;
			if(tflag) write(1,u,1);
			u++;
		}
		if(tflag) write(1," ",1);
		s = &strings[table[i].phon];
		while(*out = *s++)
			if(*out!=001) out++;
			else	{	/*do replacement*/
				u = s;
				while(*u) u++;
				while(--u>=s) *--t = *u;
				break;
			}
	}
done:
	*out = 0;
	return(out);
}

char
finals(in,ls)
char *in,**ls;
{
	char register *end;
	int val;
	end = *ls;
	val = 0;
	if(*end=='s'&&!oneof(end[-1],"us")) {
		*end-- = 0;
		if(*end=='\'') *end-- = 0;
		val = oneof(*end,"cfkpt")+1;
	}
	if(*end=='e'&&end[-1]=='i') {
		*end-- = 0;
		*end = 'y';
	}
	*ls = end;
	return(val);
}

void
midu(in,end)
char *in,*end;
{
	char register *s,*t;
	for(s=in;s<end-1;s++) if(*(t=s)=='u' && !oneof(s[-1],aeiou)) {
		if(oneof(s[1],aeiouwxy)) continue;
		if(s[2]=='r'&&oneof(s[1],bcdfgkpt)) s++;
		if(oneof(s[2]|040,aeiouy)) *t = 'U';
	}
	for(s=in;s<end-2;s++) if(oneof(*(t=s),aeo)) {
		if(oneof(s[1],"aeiouwxy|"))continue;
		if(th(s+1)) s++;
		if(s[2]=='r'&&s[3]=='i'&&oneof(s[1],bcdfgkpt)) s++;
		if(oneof(s[2],"ie")&&oneof(s[3]|040,aou)
		    || s[2]=='i'&&s[3]=='e'&&s[4]=='n')
			*t ^= 040;
	}
	s = in;
	if(*s=='y') s++;
	while(!oneof(*s|040,aeiouy)&&s<end) s++;
	if(oneof(*s,"iy") && oneof(s[1],aou)) *s ^= 040;
}

char *suff0[] = {
	"la",
	"el",
	"er",
	"su",
	"y",
	0 };
char *suff1[]  = {
	"elba",
	"de",
	"re",
	"gni",
	"tse",
	"ne",
	"ylba",
	"yl",
	"ro",
	"yre",
	"ye",
	"ssel",
	"ssen",
	"luf",
	"tnem",
	0 };
char *suff2[] = {
	"ci",
	"laci",
	0 };
char *suff3[] = {
	"e",
	0 };

void
finale(in,ls)
char *in,**ls;
{
	char register *t,*end,*u;
	char *s;
	char *z;
	end = *ls;
	if((*end=='e')&&vowel(in,end)<in) {
		*end = 'E';	/* monosyllable in -e */
		return;
	}
	t = suffix(in,end,suff0);
	if(t<end) t = longe(in,t);
	if(t==end||t<in||vowel(in,t)>=in||*t=='h') {
		t = end;
		while((u=suffix(in,t,suff1))!=t) {
			insert(u+1,ls);
			t = u;
		}
		if((u=suffix(in,t,suff2))!=t) {
			insert(u+1,ls);
			return;
		}
		if((u=suffix(in,t,suff3))!=t) {
			if(u[2]=='e') return;
			insert(u+1,ls);
			t = u;
		}
		if(oneof(*t,"iuy")&&vowel(in,t)<in) {
			*t ^=040;	/*monosyllables in -y, */
			return;
		}	/*perhaps suffixed*/
		if(!oneof(t[t[1]=='|'?2:1],"aeio")) return;
		t = longe(in,t);
		if(t<in || oneof(t[1],"cg")&&vowel(in,t)>=in) return;
		if(th(t+1)) {
			t[1] = 'T';
			t[2] = 'H';
		}
	}
	if((t==in||!oneof(t[-1],aeo)) && !(*t=='e'&&t[1]=='l'))
		*t ^= 040;
}

void
mide(in,ls)
char *in,**ls;
{
	char register *u,*end;
	end = *ls;
	for(u=in+3;u<end-2;u++)
		if(*u=='e') {
			if(u>in+4
			    &&syltest(u+1,"aeiouy|",end)
			    &&u[-1]=='l'
			    &&oneof(u[-2],"bdfgkpt")
			    &&oneof(u[-3],"bcdfgmnprst"))
				goto shift;
			if(syltest(u+1,"aeinoruy|",end)
			    &&!oneof(u[-1],"aehiouwxy")
			    &&oneof(u[-2],"aiouyU")
			    &&!oneof(u[-3],"aeiu")) {
				if(u[-3]!='o') u[-2] &= ~040;
			}
			else return;
shift:		
			insert(u+1,ls);
		}
}

void
mids(in,end)
char *in,*end;
{
	while(++in<end)
		if(*in=='s'&&oneof(in[-1]|040,"aeiouy")
		    &&oneof(in[1]|040,"aeimouy"))
			*in ^= 040;
}

int
syltest(in,s,end)
char *in,*end,*s;
{
	if(!oneof(*in|040,s))
		while(++in<end) {
			if(*in=='e'&&in[1]=='|') break;
			if(*in=='|') break;
			if(oneof(*in|040,aeiouy)) return(1);
		}
	return(0);
}

void
insert(in,ls)
char *in,**ls;
{
	char register *s,*end;
	end = *ls;
	if(*in=='e') if(*++in=='|') return;
	for(s=++end;s>=in;s--) s[1] = *s;
	*in = '|';
	*ls = end;
}

char *
suffix(in,end,s)
char *in,*end,**s;
{
	char register *t,*u;
	while(u = *s) {
		t = end+1;
		while(*u == *--t) u++;
		if(*u==0) {
			if(vowel(in,t+1)<in) break;
			else return(t);
		}
		s++;
	}
	return(end);
}

char *
longe(in,end)
char *in, *end;
{
	if(th(end-1)) end--;
	return(!oneof(*end|040,aeiouwxy)&&oneof(*--end,aeiouy)?end:in-1);
}

int
oneof(c,l)
char c,*l;
{
	while(*l) if(c == *l++) return(1);
	return(0);
}

char *
vowel(in,end)
char *in,*end;
{
	while(--end>=in)
		if(oneof(*end|040,aeiouy)) break;
	return(end);
}

int
fold(s)
char *s;
{
	if('A'<=*s && *s <='Z') {
		*s ^= 040;
		return(1);
	}
	return(0);
}

int
th(s)
char *s;
{
	return(*s=='t'&s[1]=='h');
}
