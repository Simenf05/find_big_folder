import os


size: int = 0
rootPath: str = r'C:/Users/Simen/AppData'



for path, dirs, files in os.walk(rootPath):
    
    for f in files:
        fp = os.path.join(path, f)
        size += os.path.getsize(fp)


print(f'Folder size: {size}')
