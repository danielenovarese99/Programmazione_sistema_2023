#include <stdio.h>
#include <string.h>
typedef struct{
    int type;
    float val;
    long timestamp;
} ValueStruct; /// struct containing single value

typedef struct{
    int type; // 1
    float val[10];
    long timestamp;
} MValueStruct; /// struct containing multiple values

typedef struct{
    int type; // 2
    char message[21];
} MessageStruct; /// struct containing string message

typedef struct{
    int type;
    union { /// this can contain only 1 of the 3 possible values
        ValueStruct val; // 3
        MValueStruct  mvals;
        MessageStruct messages;
    };
} ExportData;


void export2(ExportData *data, int n, FILE *pf) {
    fwrite(data, sizeof(ExportData),1,pf);
}


void export(ExportData *data, int n, FILE *pf) {
    for(int i = 0; i < n; i++)
    {
        if(data[i].type == 1){
            fwrite(&data[i].val,sizeof(ValueStruct),1,pf);
        }
        else if(data[i].type == 2)
        {
            fwrite(&data[i].mvals,sizeof(MValueStruct),1,pf);
        }
        else{
            fwrite(&data[i].messages,sizeof(MessageStruct),1,pf);
        }
    }
}
typedef struct Point{
    int x;
    int y;
}Point;

int main() {
    FILE *fp;
    ExportData data[100];

    for(int i = 0; i < 100; i++)
    {
        /// generate random shit
        if(i % 2 == 0)
        {
            /// single value float
            data[i].type = 1;
            data[i].val.type = 1;
            data[i].val.val = 20.5555;
            data[i].val.timestamp = 10000;
        }
        else if(i % 3 == 0)
        {
            /// multiple value float
            data[i].type = 2;
            data[i].mvals.type = 2;
            for(int j = 0; j < 10; j++)
                data[i].mvals.val[j] = 1.2;
            data[i].mvals.timestamp = 20000;
        }
        else {
            /// message
            data[i].type = 3;
            data[i].messages.type = 3;
            strcpy(data[i].messages.message, "Sample String");
        }
    }
    fp = fopen("outputfile2.bin","wb");
    if(fp == NULL)
    {
        printf("Error, could not open file.");
    }
    else
    {
        export(data,100,fp);
         //export2(data,100,fp);
         fclose(fp);

    }

    /*
     * int numbers[5] = {1,2,3,4,5};
    fwrite(numbers,sizeof(int),5,fp);
     * export(data,100,fp);
    fclose(fp);
     */
    return 0;
}
