[CmdletBinding()]
param (
    [Parameter()]
    [string]$EnvFile = ".env.secrets",
    [Parameter()]
    [string]$Environment = "production"
)

# Set strict mode and error handling
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Import-EnvSecrets {
    param (
        [string]$FilePath
    )
    
    if (!(Test-Path $FilePath)) {
        Write-Warning "Environment file not found: $FilePath"
        return $false
    }

    try {
        Get-Content $FilePath | ForEach-Object {
            if ($_ -match '^([^#][^=]+)=(.*)$') {
                $key = $matches[1].Trim()
                $value = $matches[2].Trim()
                
                # Set environment variable at process level
                [Environment]::SetEnvironmentVariable($key, $value, 'Process')
                Write-Verbose "Loaded secret: $key"
            }
        }
        return $true
    }
    catch {
        Write-Error "Failed to load secrets: $_"
        return $false
    }
}

# Load environment-specific secrets
$envSpecificFile = ".env.${Environment}.secrets"
if (Test-Path $envSpecificFile) {
    Write-Host "Loading environment-specific secrets from $envSpecificFile"
    Import-EnvSecrets -FilePath $envSpecificFile
}

# Load common secrets
if (Import-EnvSecrets -FilePath $EnvFile) {
    Write-Host "Successfully loaded secrets from $EnvFile"
}
else {
    Write-Warning "No secrets were loaded"
}