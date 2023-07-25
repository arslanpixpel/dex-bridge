REM Install typegen code generator
call npm install --global openapi-client-axios-typegen
REM Run typegen on the swagger json specs, write to AxiosClient class file
call typegen src/api-query/__generated__/openapi.json >> ./src/api-query/__generated__/AxiosClient.ts
REM Rename the default Client export to AxiosClient
powershell -Command "(gc src/api-query/__generated__/AxiosClient.ts) -replace 'export type Client', 'export type AxiosClient' | Out-File -encoding ASCII src/api-query/__generated__/AxiosClient.ts"
REM Export Components namespace
powershell -Command "(gc src/api-query/__generated__/AxiosClient.ts) -replace 'declare namespace Components', 'export declare namespace Components' | Out-File -encoding ASCII src/api-query/__generated__/AxiosClient.ts"
REM Export Paths namespace
powershell -Command "(gc src/api-query/__generated__/AxiosClient.ts) -replace 'declare namespace Paths', 'export declare namespace Paths' | Out-File -encoding ASCII src/api-query/__generated__/AxiosClient.ts"

