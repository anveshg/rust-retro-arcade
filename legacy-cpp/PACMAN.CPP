#include<iostream.h> //* stands for the score rectangle co ordinates
#include<stdio.h>                  //--stands for left row
#include<graphics.h>
#include<time.h>
#include<dos.h>                                   //freeze monster
#include<stdlib.h>
#include<math.h>
#include"i:\test\credits.cpp"
#include<conio.h> //if u change radius change all in w a s d.
int x=50,y=50;
int fus=0;
int mx=200,my=90;
int mx2=600,my2=440;
int re=10;
int re2=10;
int speed=5;
void dis(int x1,int y1,int r)
{

setfillstyle(SOLID_FILL,0);
setcolor(4);
circle(x1,y1,r);
floodfill(x1,y1,4);
setcolor(0);
circle(x1,y1,r);

}
int mons()
{

setcolor(14);
if(fus==9)
	speed=9;
 dis(mx,my,10);
if(mx<x)
	{
	if((mx+speed+10)>620||((mx+speed+10)<=480&&(mx+speed+10)>=180&&my+10<=330&&my+10>=150)||(((mx+speed+10>=530&&mx+speed+10<=590)||mx+speed+10>=68&&mx+speed+10<=133)&&(my+10>=240&&my+10<=242))||(((mx+speed+10)>=320&&mx+speed+10<=335)&&(my+10>=70&&my+10<=130))||(mx+speed+10>=320&&mx+speed+10<340)&&(my+10>=350&&my+10<=400))
	{

	     //	cout<<"LOL1";
	//     my+=5;
	  //   mx+=5;
	  //   my+10+=5;
	}
	else if(abs(x-mx)<5)
		mx+=1;
	else
		mx+=speed;
	}
if(mx>x)
	{
	if((mx-speed-10)<15||((mx-speed-10)<=480&&(mx-speed-10)>=180&&my<=330&&my>=150)||(((mx-speed-10>=530&&mx-speed-10<=590)&&(my>=230&&my<=250)||(mx-speed-10>=68&&mx-speed-10<=133)&&(my>=230&&my<=250)))||(((mx-speed-10)>=320&&mx-speed-10<=340)&&(my>=70&&my<=130))||(mx-speed-10>=320&&mx-speed-10<350)&&(my>=350&&my<=400))
	{
		  //  cout<<"lol2";
		//  my+=5;
		  //mx+=5;
	}
	else if(abs(mx-x)<5)
		mx-=1;
	else
		mx-=speed;
	}
if(my>y)
	{
	if((my-speed-10)<11||((mx>=70&&mx<=130)&&(my-speed-10<=252&&my-speed-10>=249))||((my-speed-10)>=150&&(my-speed-10)<=330&&(mx>=180&&mx<=480))||(my-speed-10==250&&(mx>=530&&mx<=590))||((mx>=320&&mx<=340)&&(my-speed-10>=70&&my-speed-10<=130))||mx>=320&&mx<=340&&(my-speed-10>=350&&my-speed-10<=400))
	{
	       //	cout<<"lol3";

	}
	else if(abs(y-my)<5)
		my-=1;
	else
		my-=speed;
			}
if(my<y)
	{
	if((my+speed+10)>470||((mx>=70&&mx<=130)&&my+speed+10==230)||((my+speed+10)>=150&&(my+speed+10)<=320&&(mx>=180&&mx<=480))||((my+speed+10>=230&&my+speed+10<=250)&&(mx>=530&&mx<=590))||((mx>=339&&mx<=341)&&(my+speed+10>=70&&my+speed+10<=130))||(mx>=320)&&mx<=340&&(my+speed+10>=350&&my+speed+10<=400))
	{

		//cout<<"lol4 ";
	      //	my+=5;
		//mx+=5;
	}

	else if(abs(my-y)<5)
		my+=1;
	else
		my+=speed;
	}

setcolor(14);
circle(mx,my,10);
if(mx==x&&my==y)
	return 0;
else
	return 100;
/*cout<<"  "<<mx<<" "<<x;
cout<<"  "<<my<<" "<<y;*/
}

void mons2()
{

setcolor(14);

 dis(mx2,my2,10);
// int un=0;
/*if(abs(mx2-mx)<=10||abs(my2-my)<=10)
	un=1;*/

if(mx2<x)
	{
	if((mx2+speed+10)>620||((mx2+speed+10)<=480&&(mx2+speed+10)>=180&&my2<=330&&my2>=150)||(((mx2+speed+10>=530&&mx2+speed+10<=590)||mx2+speed+10>=70&&mx2+speed+10<=130)&&(my2>=230&&my2<=250))||(((mx2+speed+10)>=320&&mx2+speed+10<=335)&&(my2>=70&&my2<=130))||(mx2+speed+10>=320&&mx2+speed+10<340)&&(my2>=350&&my2<=400))
	{
	     //	cout<<"LOL1";
	    // my2+=5;
	     //mx2+=5;
	  //   my2+=5;
	}
	else if(abs(x-mx2)<5)
		mx2+=1;
	else
		mx2+=speed;
	}
if(mx2>x)
	{
	if((mx2-speed-10)<15||((mx2-speed-10)<=480&&(mx2-speed-10)>=180&&my2<=330&&my2>=150)||(((mx2-speed-10>=530&&mx2-speed-10<=590)&&(my2>=230&&my2<=250)||(mx2-speed-10>=70&&mx2-speed-10<=130)&&(my2>=230&&my2<=250)))||(((mx2-speed-10)>=320&&mx2-speed-10<=340)&&(my2>=70&&my2<=130))||(mx2-speed-10>=320&&mx2-speed-10<350)&&(my2>=350&&my2<=400))
	{
		  //  cout<<"lol2";
	       //	  my2+=5;
		 // mx2+=5;
	}
	else if(abs(mx2-x)<5)
		mx2-=1;
	else
		mx2-=speed;
	}
if(my2>y)
	{
	if((my2-speed-10)<11||((mx2>=70&&mx2<=130)&&(my2-speed-10<=252&&my2-speed-10>=249))||((my2-speed-10)>=150&&(my2-speed-10)<=330&&(mx2>=180&&mx2<=480))||(my2-speed-10==250&&(mx2>=530&&mx2<=590))||((mx2>=320&&mx2<=340)&&(my2-speed-10>=70&&my2-speed-10<=130))||mx2>=320&&mx2<=340&&(my2-speed-10>=350&&my2-speed-10<=400))
	{
	       //	cout<<"lol3";
	      // my2+=5;
	      // mx2+=5;
	}
	else if(abs(y-my2)<5)
		my2-=1;
	else
		my2-=speed;
			}
if(my2<y)
	{
	if((my2+speed+10)>465||((mx2>=70&&mx2<=130)&&my2+speed+10==230)||((my2+speed+10)>=140&&(my2+speed+10)<=320&&(mx2>=180&&mx2<=480))||((my2+speed+10>=230&&my2+speed+10<=250)&&(mx2>=530&&mx2<=590))||((mx2>=320&&mx2<=340)&&(my2+speed+10>=70&&my2+speed+10<=130))||(mx2>=320)&&mx2<=340&&(my2+speed+10>=350&&my2+speed+10<=400))
	{

		//cout<<"lol4 ";
	      //	my2+=5;
	      //	mx2+=10;
	}

	else if(abs(my2-y)<5)
		my2+=1;
	else
		my2+=speed;
	}
/*if(abs(mx2-mx)<=10||abs(my2-my)<=10)
	return 20;*/


setcolor(14);
circle(mx2,my2,10);

if(mx2==x&&my2==y)
	exit(0);
/*cout<<"  "<<mx2<<" "<<x;
cout<<"  "<<my2<<" "<<y;*/
}
void gp()
{
	int tim;
      /*	cout<<"Press keys very hard ";
	cout<<endl<<endl<<"Press escape key to stop the game.R to restart"; */
	int gd=DETECT,gm;
	int che=0;
	int l1;
      //	sleep(5);
      initgraph(&gd,&gm,"d:\\Tc\\bgi ") ;
	cleardevice();
	int sp1=0;
	do
	{
	gotoxy(1,1);cout<<"Enter which speed u want 1/2/3/4 ";

	cin>>l1;
	if(l1>4||l1<1)
	{
	      cout<<"Please enter correct choice ";
	      getch();
	      sp1=1;
	      cleardevice();
	}
	else 	sp1=0;
	}while(sp1==1);
	clrscr();
	clock_t start,start2, end,end2;
	start = clock();
	start2=clock();

	clrscr();

	settextstyle(GOTHIC_FONT,HORIZ_DIR,5);
	cleardevice();
	double score=0;
	char ch;
	int flag=0;
	 int r=10;
	int x1[16]={100,130,160,190,220,250,280,310,350,380,410,440,470,500,530,560};
	int y1[10]={100,130,160,190,220,260,290,320,350,380};
	int x2[16]={100,130,160,190,220,250,280,310,350,380,410,440,470,500,530,560};
	int y2[10]={100,130,160,190,220,260,290,320,350,380};
	setcolor(3);   //color of blocking lines
	unsigned key;
	line(70,240,130,240); //left
	line(70,241,130,241);
	line(530,240,590,240);      //right
	line(530,241,590,241);
	line(331,70,331,130);   //up
	line(330,70,330,130);
	line(330,350,330,400);      //down
	line(331,350,331,400);
	setcolor(13);
	rectangle(190,155,470,320); //smaller rect score.*************
	setcolor(13);
	rectangle(0,0,639,479); //border
	setcolor(4);
	setfillstyle(1,4);
	pieslice(270,230,320,20,60);
	floodfill(270,230,4);
	do
	{

		flag++;
		settextstyle(1,HORIZ_DIR,2);
		setcolor(10);
		outtextxy(320,160,"GOBBLE MOUTH ");
		setcolor(9);


		outtextxy(340,220,"   LEVEL 1.");
		gotoxy(48,17);
		cout<<"SCORE:"<<score;
		gotoxy(48,18);
		end2=clock();
						//asjdgasdgiyagsdiygdsydagsy
		int suh= (end2-start2) /CLK_TCK;
		cout<<"Time:"<<  suh;

		if(flag%2==0)
			{
			       setcolor(14);
				setfillstyle(SOLID_FILL,14);
				pieslice(x,y,320,21,r);
			       floodfill(x,y,14);
				 setcolor(0);     //for eyes
				circle(x+2,y-5,1);

			}
	else
	       {        setcolor(14); setfillstyle(SOLID_FILL,14);
		     circle(x,y,r);floodfill(x,y,14);
		      setcolor(0);
			circle(x+2,y-5,1);     //eye
			line(x+2,y,x+10,y);         //mouth
	       }
	       if(che==3)
			{
				cleardevice();
				break;
			}

		      re=mons();
		      if(re==0)
			{
			setcolor(5);
			circle(mx,my,10);
			setfillstyle(SOLID_FILL,0);
			floodfill(mx,my,5);
			setcolor(0);
			circle(mx,my,10);
			break;
			}

			mons2();
		       //	int mus=0;
		       /*	if(mx2==mx&&my2==my&&mus==0)
			     {                     mus++;
						       // sound();
				fus=9;
			     }    */
	if(abs(y-100)<=r)
	{
	for(int i1=0;i1<16;i1++)
	{
		if(abs(x1[i1]-x)<=r+2)
		       {
		       sound(500);
		       delay(20);
		       nosound();
		       dis(x1[i1],100,2);
		       setcolor(9);
			x1[i1]=-100;
			score+=10;
			}

	}
	}
	if(abs(x-100)<=r)
	{
	for(int i1=0;i1<10;i1++)  //--
	{
		if(abs(y1[i1]-y)<=r+2)
		{
		sound(500);
		       delay(20);
		       nosound();
		dis(100,y1[i1],2);
		       setcolor(9);
		y1[i1]=-100;
		score+=10;
		}
	}
	}
	if(abs(x-560)<=r)
	{
	for(int i1=0;i1<10;i1++)  //--
	{
		if(abs(y2[i1]-y)<=r+2)
		{
		sound(500);
		       delay(20);
		       nosound();
		dis(560,y2[i1],2);
		       setcolor(9);
		y2[i1]=-100;
		score+=10;
		}
	}
	}

	if(abs(y-380)<=r)
	{
	for(int i1=0;i1<16;i1++)
	{
		if(abs(x2[i1]-x)<=r+2)
		{
		sound(500);
		       delay(20);
		       nosound();
		dis(x2[i1],380,2);
		       setcolor(9);
		x2[i1]=-100;
		score+=10;
		}
	}
	}
	if(y==400&&x==600)
	{
	che++;
	}
      /*	setcolor(14);
	settextstyle(1,HORIZ_DIR,5);
	outtextxy(600,400,"L");*/
	for(int i=0;i<16;i++)
	{
	       setcolor(9);circle(x1[i],100,2);
	}  //y cordinate of small circle
	for(i=0;i<10;i++)             //--
	{
		circle(100,y1[i],2);//left row
	}
	for(i=0;i<10;i++)             //--
	{
		circle(560,y2[i],2);//left row
	}
	for(i=0;i<16;i++)
	{
	       setcolor(9);circle(x2[i],380,2);
	}  //y co0rdinate of small circle
	key=inp(0x60);
	if(l1==1)
		delay(200);
	else if(l1==2)
		delay(90);
	else if(l1==3)
		delay(50);
	else
		delay(5);

	if(key==0x4b)
		ch='a';
	else if(key==0x4d)
		ch='d';
	else if(key==0x48)
		ch='w';
	else if(key==0x50)
		ch='s';
	else if(key==0x81)
		ch='0';
	else if(key==0x93)
		ch='r';

	 dis(x,y,r);

	if(ch=='w')
	      {
		if((y-r)<11||((x>=70&&x<=130)&&(y-r<=252&&y-r>=249))||((y-r)>=150&&(y-r)<=330&&(x>=180&&x<=480))||(y-r==250&&(x>=530&&x<=590))||((x>=320&&x<=340)&&(y-r>=70&&y-r<=130))||x>=320&&x<=340&&(y-r>=350&&y-r<=400))    //*
		{
		flag-- ;
		}
		else
			y-=10;
			}
	else if(ch=='s')
		{

		if((y+r)>465||((x>=70&&x<=130)&&y+r==230)||((y+r)>=150&&(y+r)<=320&&(x>=180&&x<=480))||((y+r>=230&&y+r<=250)&&(x>=530&&x<=590))||((x>=320&&x<=340)&&(y+r>=70&&y+r<=130))||(x>=320)&&x<=340&&(y+r>=350&&y+r<=400))
		{
		flag-- ;
		} //*

		else
			y+=10;
		}
	else if(ch=='a')
		{
		if((x-r)<15||((x-r)<=480&&(x-r)>=180&&y<=330&&y>=150)||(((x-10>=530&&x-10<=590)&&(y>=230&&y<=250)||(x-10>=70&&x-10<=130)&&(y>=230&&y<=250)))||(((x-r)>=320&&x-r<=340)&&(y>=70&&y<=130))||(x-10>=320&&x-10<350)&&(y>=350&&y<=400))     //*
		{
		flag-- ;
		}
		else
			x-=10;
		}
	else if(ch=='d')
		{
		if((x+r)>620||((x+r)<=480&&(x+r)>=180&&y<=330&&y>=150)||(((x+10>=530&&x+10<=590)||x+10>=70&&x+10<=130)&&(y>=230&&y<=250))||(((x+10)>=320&&x+10<=335)&&(y>=70&&y<=130))||(x+10>=320&&x+10<340)&&(y>=350&&y<=400))    //*
		 {
		flag-- ;
		}
		else
			x+=10;
		 }
	else  if(ch=='0')
		  break;
	else if(ch=='r')
		exit(0);
	else
		    flag-- ;

	}while(score!=520);
       if(che==3)
       {
	cleardevice();
	credit();
	sleep(5);
	exit(0);
       }
       if(l1==2)                                   //ajsdhaljdhdasuhdsuh
		score+=25;
       if(l1==3)
		score+=35;
       if(l1==4)
		score+=40;
       end = clock();
	 sleep(1);
     cleardevice();
	 gotoxy(47,16);

     if(key!=0x81&&re!=0)
     {
	 setcolor(14);
	 settextstyle(3,0,2);
	 outtextxy(218,230," The time was: ");
	 int g=100-(end-start) / CLK_TCK;

	 cout<<(end-start)/CLK_TCK;
	 outtextxy(430,230,"  seconds ");
	     sleep(3);
	      cleardevice();
    outtextxy(250,230," Your score was ");
      cout<<floor(score+g);
    }
	else
	{
  //	cleardevice();
  setcolor(14);
	settextstyle(3,0,2);
	outtextxy(200,230," Your score was    ");
	cout<<score;
	sleep(2);
	getch();
	}
	sleep(2);
	}