# package-vsix.ps1
# Packages the SYNX Visual Studio extension into a valid .vsix file.
# A .vsix is an OPC (Open Packaging Convention) ZIP with specific required parts.

param(
    [string]$BuildOutput = "SynxLanguageService\bin\Release\net472",
    [string]$Manifest    = "SynxLanguageService\source.extension.vsixmanifest",
    [string]$OutFile     = "SynxLanguageService.vsix"
)

$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
Push-Location $scriptDir

try {
    $buildOut = Resolve-Path $BuildOutput
    $manifest = Resolve-Path $Manifest
    $outPath  = Join-Path $scriptDir $OutFile

    Write-Host "[package] Build output : $buildOut"
    Write-Host "[package] Manifest     : $manifest"

    if (Test-Path $outPath) { Remove-Item $outPath -Force }

    $zip = [System.IO.Compression.ZipFile]::Open($outPath, [System.IO.Compression.ZipArchiveMode]::Create)

    function Add-ZipText([System.IO.Compression.ZipArchive]$archive, [string]$entryName, [string]$text) {
        $entry  = $archive.CreateEntry($entryName, [System.IO.Compression.CompressionLevel]::Optimal)
        $writer = New-Object System.IO.StreamWriter($entry.Open(), [System.Text.Encoding]::UTF8)
        $writer.Write($text)
        $writer.Dispose()
    }

    function Add-ZipFile([System.IO.Compression.ZipArchive]$archive, [string]$entryName, [string]$filePath) {
        $entry  = $archive.CreateEntry($entryName, [System.IO.Compression.CompressionLevel]::Optimal)
        $src    = [System.IO.File]::OpenRead($filePath)
        $dest   = $entry.Open()
        $src.CopyTo($dest)
        $src.Dispose()
        $dest.Dispose()
    }

    # 1. [Content_Types].xml  — MUST be first entry; Extension has NO leading dot (OPC spec)
    Add-ZipText $zip "[Content_Types].xml" @'
<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels"    ContentType="application/vnd.openxmlformats-package.relationships+xml" />
  <Default Extension="dll"     ContentType="application/octet-stream" />
  <Default Extension="png"     ContentType="image/png" />
  <Default Extension="svg"     ContentType="image/svg+xml" />
  <Default Extension="pkgdef"  ContentType="application/octet-stream" />
  <Override PartName="/extension.vsixmanifest" ContentType="text/xml" />
</Types>
'@

    # 2. _rels/.rels  — OPC package relationship pointing at the manifest
    Add-ZipText $zip "_rels/.rels" @'
<?xml version="1.0" encoding="utf-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Type="http://schemas.microsoft.com/developer/vsx-schema/2011" Target="/extension.vsixmanifest" Id="R1" />
</Relationships>
'@

    # 3. extension.vsixmanifest
    Add-ZipFile $zip "extension.vsixmanifest" $manifest

    # 4. Main DLL
    Add-ZipFile $zip "SynxLanguageService.dll" "$buildOut\SynxLanguageService.dll"

    # 5. Newtonsoft.Json.dll (dependency)
    $newtonsoftPath = "$buildOut\Newtonsoft.Json.dll"
    if (Test-Path $newtonsoftPath) {
        Add-ZipFile $zip "Newtonsoft.Json.dll" $newtonsoftPath
    }

    # 6. Resources (icon, svg)
    $resDir = "$buildOut\Resources"
    if (Test-Path $resDir) {
        Get-ChildItem $resDir -File | ForEach-Object {
            Add-ZipFile $zip "Resources/$($_.Name)" $_.FullName
        }
    }

    $zip.Dispose()

    $size = [math]::Round((Get-Item $outPath).Length / 1KB, 1)
    Write-Host "[package] VSIX created : $outPath" -ForegroundColor Green
    Write-Host "[package] Size         : $size KB"

} finally {
    if ($zip) { try { $zip.Dispose() } catch {} }
    Pop-Location
}
