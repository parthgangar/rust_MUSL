#include<stdio.h>
#include<stdlib.h>
#include<string.h>


int main()
{
	char s1[]="parth";
	char s2[]="paras";
	char p1[100]="GeeksFor";
	char str1[]="abc";
	char str2[]="aby";

//Here are some demos of the function that we have implemented in RUST
	
	//demo of strspn function 
	size_t n=fn_strspn(s1,s2);
	printf("%lu\n",n);
	
	//demo of memset function
	char *s3=fn_memset(s2,97,3);
	printf("%s\n",s2);//s2 is now changed
	
	//demo of bzero function
	fn_bzero(str1,5);
	printf("\n**%s**\n",s1);//s1 will now have only null characters
	
	//demo of memcpy function 
	fn_memcpy(p1+5,p1,strlen(p1)+1);
	printf("%s\n",p1);//p1 is now changed
	
	//demo of strncmp function
	int diff=fn_strncmp(str1,str2,2);
	printf("%d\n",diff);
	
	return 0;
}
