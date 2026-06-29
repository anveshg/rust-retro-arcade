#include<iostream.h>
#include<graphics.h>
#include<conio.h>
#include<dos.h>
#include"i:\test\PACMAN.cpp"
//#include"d:\agentx\test\PONG.cpp" //change x and y
#include"i:\test\DEMO.cpp"
#include"i:\test\INSTRUCT.cpp"
int i;
void start();
char menu();
void bordera(int x1,int del);
void main()
{
	int gd=DETECT,gm;
	initgraph(&gd,&gm,"c:\tc\bgi");
	bordera(14,2);
	//start();
	//sound();
	char d=menu();
	cleardevice();
/*	do
	{
		switch(d)
		{
			case 1:gp();
				break;

			case 2:anv();
				break;

			case 3:instruct();
				break;

			case 4:{
				cleardevice();
				int lc;
				outtextxy(200,200,"Enter level code : ");
				cout<<"\t\t\t";
				cin>>lc;
				if(lc==91)
					//um();
				getch();
				cleardevice();
			       }
			}
		}while(d!=5);
	*/
	if(d=='1')
		gp();
	if(d=='2')
		anv();
	if(d=='3')
		instruct();
	if(d=='4')
	{
		cleardevice();
		int lc;
		outtextxy(200,200,"Enter level code : ");
		cout<<"\t\t\t";
		cin>>lc;
		if(lc==91)
			//um();
	getch();
	cleardevice();
	}
	if(d==5)
		exit(0);
	if(d!='5'&& d!='3')
	{
	setcolor(4);
	rectangle(5,5,635,475);
	setcolor(4);
	settextstyle(6,0,3);
	outtextxy(150,180,"Do you want play again ?[Y/N]");

	char g;

       //	g=getch();

	gotoxy(23,15);
	cin>>g;
	cleardevice();
	if(g=='y'||g=='Y')
	{
	       int g=menu();
	       if(g=='1')
		gp();
	if(g=='3')
	{
		instruct();
	}
	if(g=='4')
		{
		cleardevice();
		int lc;
		outtextxy(200,200,"Enter level code : ");
		cout<<"\t\t\t";
		cin>>lc;
		//if(lc==91)
		       //	um();


		 }
	if(g=='5')
		exit(0);
	else
	cleardevice();

	setcolor(4);
	rectangle(5,5,635,475);
	outtextxy(180,200,"Thanks for playing");

	getch();


}
void bordera(int x1,int del)
{
	setcolor(x1);
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
void start()
{
       /*	setcolor(2);
	rectangle(5,5,635,475);
	setcolor(4);
	rectangle(10,10,630,470); */
	for(i=1;i<=140;i++)
	{
		setfillstyle(SOLID_FILL,4);
		setcolor(0);pieslice(320,240,10,340,i-1);setcolor(12);
		pieslice(320,240,10,340,i);
		delay(10);

	}
	setfillstyle(SOLID_FILL,0);
	setcolor(12);
	circle(390,180,10);
	floodfill(390,180,12);
	settextstyle(3,0,4);
	setcolor(15);
	setfillstyle(SOLID_FILL,14);
	setcolor(7);
	circle(430,250,10);
	floodfill(431,251,7);
	delay(500);
	setfillstyle(SOLID_FILL,4);
	setcolor(12);
	pieslice(320,240,0,360,140);
	floodfill(325,240,12);
	setfillstyle(SOLID_FILL,0);
	setcolor(12);
	circle(390,180,10);
	floodfill(390,180,12);
	settextstyle(3,0,5);
	setcolor(14);
    //	outtextxy(240,400,"PACMAN");
	//getch();
}
char menu()
{
	cleardevice();
	bordera(13,0);
	int x2=200,y=100,ch;
	setcolor(9);
	settextstyle(1,0,5);
	outtextxy(x2,y,"1.Start game");
	outtextxy(x2,y+50,"2.Demo");
	outtextxy(x2,y+100,"3.Help");
	outtextxy(x2,y+150,"4.Level Code");
	outtextxy(x2,y+200,"5.Exit");
	settextstyle(1,0,2);
	setcolor(14);
	outtextxy(x2,y+300,"Enter Choice To Continue");
       char choice=getch();
       return choice;
	//ch=getch();
	//switch(ch);

}}