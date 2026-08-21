import pefile
import glob

dlls = glob.glob(r"C:\Windows\System32\*.dll")
names = set()

for path in dlls:
    try:
        pe = pefile.PE(path, fast_load=True)
        pe.parse_data_directories(
            directories=[
                pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXPORT']
            ]
        )

        if hasattr(pe, "DIRECTORY_ENTRY_EXPORT"):
            for exp in pe.DIRECTORY_ENTRY_EXPORT.symbols:
                if not exp.name:
                    continue

                name = exp.name.decode("ascii", errors="ignore")

                if name.startswith("?"):
                    continue

                names.add(name)

    except Exception:
        pass

with open("apis.txt", "w", encoding="utf-8") as f:
    f.write("\n".join(sorted(names)))