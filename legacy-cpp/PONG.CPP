#include<iostream.h>
#include<conio.h>
#include<graphics.h>
#include<dos.h>
#include<stdlib.h>
void interrupt  (*OldKeyboard)(...);  //to store the original interrupt handler
void InitKeyboard(void);
void RestoreKeyboard(void);
void clear(int,int);
void wall(int , int);
		  char c;int r=10,xa=20,y=100,x2=620,y2=100;
void clear1(int x2,int y2)
{
	setcolor(0);
	line(x2,y2,x2,y2+60);
	line(x2-10,y2+10,x2-10,y2+50);
	line(x2,y2,x2-10,y2+10);
	line(x2,y2+60,x2-10,y2+50);
	setcolor(15);
}
void cl(int xa,int y)
{
	circle(xa,y,8);
	setfillstyle(1,0);
	floodfill(xa,y,15);
	setcolor(0);
	circle(xa,y,8);
	setcolor(15);
}
void interrupt MyKeyboard(...)   //our interrupt handler subroutine
{
	unsigned key;
	key = inp(0x60);
	outp(0x20, 0x20);
	if(key==0x48){c='w';//move();
	}
	else if(key==0x50 ){c='s';//move();
	}
	else if(key==0x4b)c='g';
	return;
}
void InitKeyboard(void)
{
	OldKeyboard = _dos_getvect(9);
	_dos_setvect(9, MyKeyboard);
	return ;
}

void RestoreKeyboard(void)
{
	_dos_setvect(9, OldKeyboard);
	return;
}
void um()
{

	int gd=DETECT,gm;
	initgraph(&gd,&gm,"c:\\tc\\bgi");
	int x1=600,y1=300;
	setcolor(1);
	setfillstyle(1,1);
	rectangle(0,0,640,9);
	floodfill(50,5,1);
	rectangle(0,452,640,462);
	floodfill(50,455,1);
	rectangle(0,10,9,40);
	floodfill(5,30,1);
	rectangle(0,431,9,451);
	floodfill(5,440,1);
	rectangle(631,10,640,40);
	floodfill(635,30,1);
	rectangle(631,431,640,451);
	floodfill(635,440,1);
	gotoxy(25,30);
	cout<<"SPEED ";
	rectangle(240,465,370,475);
	setcolor(15);
	line(15,464,15,480);
	line(20,469,20,475);
	line(15,464,20,469);
	line(15,480,20,475);
	line(25,464,25,480);
	line(30,469,30,475);
	line(25,464,30,469);
	line(25,480,30,475);
	delay(1000);
	long n=0,n1=1,sp=0,sp1=1;
	unsigned key;
	int d=0,d1=2,s=25,p=241,sc=0,r=1,o=0;
	int Y[]={20,150,250,300,350,410,450};
	char comment[5][15]={"Not Bad..","Nice..","Good!!","That's Cool :)"
	,"Scholar!!!"};
	randomize();
	y1=Y[random(7)];
	InitKeyboard();
	settextstyle(7,0,6);

	outtextxy(300,200,"3");
	delay(1000);
	setcolor(0);
	outtextxy(300,200,"3");
	setcolor(15);
	outtextxy(300,200,"2");
	delay(1000);
	setcolor(0);
	outtextxy(300,200,"2");
	setcolor(15);
	outtextxy(300,200,"1");
	delay(1000);
	setcolor(0);
	outtextxy(300,200,"1");
	setcolor(15);
	int life=0,l=0;
	for(;;)
	{
		if(p>=370)
		{
			p=241;setcolor(0);
			for(int j=241;j<=369;j++)
				line(j,466,j,474);
			setcolor(15);
			if(r){s=10;n1=3;sp1=2;}
			else r=1;
		}
		if(n%n1==0)
		{
			line(p,466,p,474);
			p++;
		}
		if(n==9000)
			{n=0;cout<<"yes";}
		n++;sp++;
		gotoxy(67,30);
		cout<<"SCORE :"<<sc;
		line(xa,y,xa,y+60);
		line(xa+10,y+10,xa+10,y+50);
		line(xa,y,xa+10,y+10);
		line(xa,y+60,xa+10,y+50);
		line(x2,y2,x2,y2+60);
		line(x2-10,y2+10,x2-10,y2+50);
		line(x2,y2,x2-10,y2+10);
		line(x2,y2+60,x2-10,y2+50);
		setcolor(1);
		circle(x1,y1,8);setcolor(15);
		setfillstyle(1,3);
		floodfill(x1,y1,1);
		delay(s);
		if(d==0 && ( ((x1>=x && x1<=x+20) && (y1>=y && y1<=y+60))
		|| ((x1<=20 && y1<=40) || (x1<=20 && y1>=430)) ))
		{
			if(y1==y+30)
			{
				d1=4;
			}
			else if(y1==y+29 || y1==y+31)
			{
				d1=6;
			}
			else if(y1==y+27 || y1==y+28 || y1==y+32 || y1==y+33)
			{
				d1=8;
			}
			if((y1>=y && y1<=y+26) && (y1>=y+34 && y1<=y+60))
			{
				d1=10;
			}
			d=1;sc+=10;s=25;n1=2;sp1=1;
		}
		if(d==1 && (((x1>=x2-10 && x1<=x2) && (y1>=y2 && y1<=y2+60))
		|| ((x1>=620 && y1<=50) || (x1>=620 && y1>=410))
		|| x1>=660))
		{
			if(x1>=660)
			{
				sc+=100;
				settextstyle(7,0,6);
				outtextxy(250,190,comment[o]);
				delay(1000);
				setcolor(0);
				outtextxy(250,190,comment[o]);
				setcolor(15);
				p=370;r=0;x1=600; y1=280;
				if(o<=3)o++;
			}
			if(y1==y2+30)
			{
				d1=4;
			}
			else if(y1==y2+29 || y1==y2+31)
			{
				d1=6;
			}
			else if(y1==y2+27 || y1==y2+28 || y1==y2+32 || y1==y2+33)
			{
				d1=8;
			}
			if((y1>=y2 && y1<=y2+26) && (y1>=y2+34 && y1<=y2+60))
			{
				d1=10;
			}
			d=0;
		}
		if(d1==2 && y1<=20)
		{
			d1=3;
		}
		if(d1==3 && y1>=440)
		{
			d1=2;
		}
		if(d1==4 && y1<=20)
		{
			d1=5;
		}
		if(d1==5 && y1>=440)
		{
			d1=4;
		}
		if(d1==6 && y1<=20)
		{
			d1=7;
		}
		if(d1==7 && y1>=440)
		{
			d1=6;
		}
		if(d1==8 && y1<=20)
		{
			d1=9;
		}
		if(d1==9 && y1>=440)
		{
			d1=8;
		}
		if(d1==10 && y1<=20)
		{
			d1=11;
		}
		if(d1==11 && y1>=440)
		{
			d1=10;
		}
		if(life==3)
			c='g';//break;
		if(c=='g')
			break;
		cl(x1,y1);
		if(d==1)
		{
			x1+=10;
		}
		else
		{
			x1-=10;
		}
		if(d1==2)
		{
			y1-=10;
		}
		else if(d1==3)
		{
			y1+=10;
		}
		else if(d1==4)
		{
			y1-=1;
		}
		else if(d1==5)
		{
			y1+=1;
		}
		else if(d1==6)
		{
			y1-=3;
		}
		else if(d1==7)
		{
			y1+=3;
		}
		else if(d1==8)
		{
			y1-=6;
		}
		else if(d1==9)
		{
			y1+=6;
		}
		else if(d1==10)
		{
			y1-=30;delay(10);
		}
		else if(d1==11)
		{
			y1+=30;delay(10);
		}
		if(x1>=400)
		{
			if(y2+30<y1)
			{
				clear1(x2,y2);
				if(y2<=380) //360
				y2+=10;
			}
			else
			{
				clear1(x2,y2);
				if(y2>=20) //40
				y2-=10;
			}
		}
		if(x1<0)
		{
			l=1;
		}
		if(l)
		{
			life++;l=0;
			x1=600;delay(2000);p=370; r=0; s=25;
		}
		if(life==1)
		{
			setcolor(0);
			line(25,464,25,480);
			line(30,469,30,475);
			line(25,464,30,469);
			line(25,480,30,475);
			setcolor(15);
		}
		if(life==2)
		{
			setcolor(0);
			line(15,464,15,480);
			line(20,469,20,475);
			line(15,464,20,469);
			line(15,480,20,475);
			setcolor(15);
		}
		if(sp%sp1==0)
		{
			if((c=='w' && y<=10) || (c=='s' && y>=390))
			continue;
			if(c=='w')
			{
				clear(x,y);
				y=y-10;
			}
			else if(c=='s')
			{
				clear(x,y);
				y=y+10;
			}
		}
	}
	settextstyle(4,HORIZ_DIR,6);
	outtextxy(180,200,"GameOver");
	RestoreKeyboard();
}
void clear(int x,int y)
{
	setcolor(0);
	line(x,y,x,y+60);
	line(x+10,y+10,x+10,y+50);
	line(x,y,x+10,y+10);
	line(x,y+60,x+10,y+50);
	setcolor(15);
}