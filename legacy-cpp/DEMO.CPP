#include<iostream.h>
#include<graphics.h>
#include<conio.h>
#include<dos.h>
#include<stdio.h>
#include<time.h>
#include<stdlib.h>
int xa=320,ya=180;
int flag=1,sc=0;
char ch;
void blk(int x,int ya,int r);
void wall();
void pacman(int x,int ya);
void sound();
void border(int clr,int del);
void interrupt  (*OldKeyboard)(...);
void interrupt MyKeyboard(...);
void RestoreKeyboard();
void InitKeyboard();
void demo(int r);
void circle();
void anv()
{
	int r=10;
	clrscr();
	int gd=DETECT,gm;
	initgraph(&gd,&gm," ");
	//InitKeyaboard();
	border(4,0);//color,delaya
	wall();
	/*do
	{
		sound();
	}while(flag=1);*/
	rectangle(250,200,390,240);
	demo(10);
	//RestoreKeyaboard();

	closegraph();
}
void demo(int r)
{
	//going right
	for(;xa<=560;xa+=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going up
	for(;ya>=80;ya-=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}

	//going left and stopping at gap
	for(;xa>=330;xa-=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going up thru gap
	for(;ya>=40;ya-=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going left
	for(;xa>=40;xa-=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going down
	for(;ya<=440;ya+=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going right
	for(;xa<=600;xa+=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}
	//going up
	for(;ya>=30;ya-=10)
	{
		pacman(xa,ya);
		delay(50);
		blk(xa,ya,r);
	}

	flag=0;
}
void interrupt MyKeyboard(...)   //our interrupt handler subroutine
{
	//demo();
	sound();
	return;
}
void InitKeyboard()
{
	OldKeyboard = _dos_getvect(9);
	_dos_setvect(9, MyKeyboard);
	return ;
}

void RestoreKeyboard()
{
	_dos_setvect(9, OldKeyboard);
	return;
}
void pacman(int x,int ya)
{
			//pacman
			setcolor(14);
			setfillstyle(SOLID_FILL,14);
			pieslice(x,ya,20,340,10);
			floodfill(x-2,ya-2,14);
			//eyae
			setcolor(0);
			setfillstyle(SOLID_FILL,0);
			circle(x+2,ya-5,2);
			setfillstyle(SOLID_FILL,0);
			floodfill(x+2,ya-5,0);

}
void wall()
{
	setcolor(11);
	line(50,50,300,50);
	line(340,50,590,50);
	line(50,430,300,430);
	line(340,430,590,430);
	line(50,50,50,430);
	line(590,50,590,430);
	//x1+=40;ya1+=40;x2-=40;ya2-=40;
	//x1=50,x2=590,ya1=50,ya2=435;
	line(50,50,300,50);
	line(340,50,590,50);
	line(50,430,300,430);
	line(340,430,590,430);
	line(50,50,50,430);
	line(590,50,590,430);


}
void sound()
{
	for(int i=0;i<1;i++)
	{
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(700);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(900);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(700);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(900);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(600);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(700);
	delay(200);
	nosound();
	sound(7);
	delay(200);
	nosound();
	sound(400);
	delay(200);
	nosound();
	}
	return;
}
void border(int clr,int del)
{
	setcolor(clr);
	for(int i=0;i<640;i++)
	{
		arc(i,4,0,360,4);
		delay(del);
	}
	for(int j=0;j<480;j++)
	{
		arc(635,j,0,360,4);
		delay(del);
	}
	for(int k=635;k>0;k--)
	{
		arc(k,475,0,360,4);
		delay(del);
	}
	for(int l=475;l>0;l--)
	{
		arc(4,l,0,360,4);
		delay(del);
	}
}
void blk(int x,int ya,int r)
{
	//Pacman
	setfillstyle(SOLID_FILL,0);
	setcolor(0);
	circle(x,ya,r);
	floodfill(x,ya,0);
	setcolor(0);
	circle(x,ya,r);
	//Eyae
	setfillstyle(SOLID_FILL,0);
	setcolor(0);
	circle(x,ya-5,r);  //x+2,ya-5
	floodfill(x+1,ya-6,0);//x+1,ya-4
	setcolor(0);
	circle(x,ya-5,r);
}
