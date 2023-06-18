import os
import time

size: int = 0
rootPath: str = r'../'


start = time.time()
for path, dirs, files in os.walk(rootPath):
    
    for f in files:
        fp = os.path.join(path, f)
        size += os.path.getsize(fp)
end = time.time()

elapsed = end - start

print(f'Folder size: {size} and took {elapsed}')
