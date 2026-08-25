import pefile
import os

dlls = [
    "ntdll.dll",
    "kernel32.dll",
    "kernelbase.dll",
    "advapi32.dll",
    "user32.dll",
    "shell32.dll",
    "ole32.dll",
    "oleaut32.dll",
    "combase.dll",
    "shlwapi.dll",
    "ws2_32.dll",
    "mswsock.dll",
    "winhttp.dll",
    "wininet.dll",
    "urlmon.dll",
    "httpapi.dll",
    "dnsapi.dll",
    "nsi.dll",
    "rasapi32.dll",
    "iphlpapi.dll",
    "netioapi.dll",
    "mpr.dll",
    "netapi32.dll",
    "wlanapi.dll",
    "fwpuclnt.dll",
    "secur32.dll",
    "sspicli.dll",
    "crypt32.dll",
    "bcrypt.dll",
    "bcryptprimitives.dll",
    "ncrypt.dll",
    "ncryptsslp.dll",
    "cryptnet.dll",
    "wintrust.dll",
    "msasn1.dll",
    "samlib.dll",
    "samsrv.dll",
    "lsasrv.dll",
    "wevtapi.dll",
    "evntrace.dll",
    "psapi.dll",
    "dbghelp.dll",
    "imagehlp.dll",
    "wtsapi32.dll",
    "userenv.dll",
    "profapi.dll",
    "setupapi.dll",
    "cfgmgr32.dll",
    "fltlib.dll",
    "propsys.dll",
    "actxprxy.dll",
    "mshtml.dll",
    "wbemcomn.dll",
    "wbemprox.dll",
    "wbemsvc.dll",
    "fastprox.dll",
    "wmiutils.dll",
    "gdi32.dll",
    "gdi32full.dll",
    "win32u.dll",
    "imm32.dll",
    "uxtheme.dll",
    "dwmapi.dll",
    "winmm.dll",
    "avrt.dll",
    "mf.dll",
    "mfplat.dll",
    "mfreadwrite.dll",
    "d3d11.dll",
    "dxgi.dll",
    "hid.dll",
    "scmapi.dll",
    "srvcli.dll",
    "clusapi.dll",
    "spoolss.dll",
    "mscoree.dll",
    "clr.dll",
    "coreclr.dll",
    "hostfxr.dll",
    "hostpolicy.dll",
    "nethost.dll",
    "twinapi.dll",
    "twinapi.appcore.dll",
]

names = set()
for dll in dlls:
    path = os.path.join(r"C:\Windows\System32", dll)
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