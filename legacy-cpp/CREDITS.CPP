#include<iostream.h>
#include<conio.h>
#include<graphics.h>
void credit()
{

	int gd=DETECT,gm;
	initgraph(&gd,&gm,"c:\Tc\Bgi ");
	setcolor(10);
	cleardevice();
	settextstyle(1,HORIZ_DIR,7);
	outtextxy(180,1,"CREDITS");
	outtextxy(210,200,"ANVESH");
	getch();

}
