#include<iostream.h>
#include<graphics.h>
#include<conio.h>
#include<dos.h>
void borderb(int clr,int del);
void instruct()
{

	int gd=DETECT,gm;
	initgraph(&gd,&gm,"c:\tc\bgi");
	cleardevice();
	borderb(5,0);
	setcolor(4);
	settextstyle(4,0,4);
	outtextxy(220,40,"INSTRUCTIONS");
	settextstyle(1,0,1);
	outtextxy(40,100,"To move press up,down,left or right keys in pacman.");
	outtextxy(40,140,"and in pong use up and down keys.");
	outtextxy(40,180,"In pong to shoot use the spacebar.");
	outtextxy(40,220,"Avoid the monster in pacman and the faster you finish");
	outtextxy(40,240,"the more the score.");
	outtextxy(220,400,"Press Any Key To Continue");
	outtextxy(220,123,"asdad");
	getch();
}
void borderb(int clr,int del)
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