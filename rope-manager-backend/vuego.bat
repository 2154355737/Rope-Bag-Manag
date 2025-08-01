@echo off
pause
chcp 963 >nul

setlocal enabledelayedexpansion

echo ������������ű�...
echo.

REM Create log directory first
set LOG_DIR=%~dp0logs
if not exist "%LOG_DIR%" (
    mkdir "%LOG_DIR%"
    echo �Ѵ�����־Ŀ¼: %LOG_DIR%
)

REM Create timestamped log file
set "timestamp=%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%"
set "timestamp=!timestamp: =0!"
set LOG_FILE=%LOG_DIR%\deploy_!timestamp!.log

echo ========== ����ű����� ========== > "%LOG_FILE%"
echo �ű�����ʱ��: %date% %time% >> "%LOG_FILE%"
echo ����Ŀ¼: %CD% >> "%LOG_FILE%"
echo ============================================== >> "%LOG_FILE%"

echo [��־] �Ѵ�����־�ļ�: %LOG_FILE%

REM Basic error handling
if not defined COMSPEC (
    echo ����: �������δ����
    echo ����: �������δ���� >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [��־] ����������ͨ�� >> "%LOG_FILE%"

REM Configuration
set FRONT_ROOT=D:\vue\zhiyuanshequ\rust\Rust_Vue
set NGINX_HOME=D:\vue\zhiyuanshequ\rust\nginx-1.28.0
set BACKEND_ROOT=D:\vue\zhiyuanshequ\rust\rope-manager-backend

echo [��Ϣ] ǰ��Ŀ¼: %FRONT_ROOT%
echo [��Ϣ] NginxĿ¼: %NGINX_HOME%
echo [��Ϣ] ���Ŀ¼: %BACKEND_ROOT%
echo.

echo [��־] ��������: >> "%LOG_FILE%"
echo [��־] FRONT_ROOT=%FRONT_ROOT% >> "%LOG_FILE%"
echo [��־] NGINX_HOME=%NGINX_HOME% >> "%LOG_FILE%"
echo [��־] BACKEND_ROOT=%BACKEND_ROOT% >> "%LOG_FILE%"

echo [��Ϣ] ��־�ļ��Ѵ���: %LOG_FILE%
echo.

REM Check directories
echo ========== ������� ==========
echo [LOG] ��ʼ������� >> "%LOG_FILE%"

if not exist "%FRONT_ROOT%" (
    echo [����] ǰ��Ŀ¼δ�ҵ�
    echo [����] ǰ��Ŀ¼δ�ҵ�: %FRONT_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] ǰ��Ŀ¼����
echo [LOG] ǰ��Ŀ¼����: %FRONT_ROOT% >> "%LOG_FILE%"

if not exist "%NGINX_HOME%" (
    echo [����] NginxĿ¼δ�ҵ�
    echo [����] NginxĿ¼δ�ҵ�: %NGINX_HOME% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] NginxĿ¼����
echo [LOG] NginxĿ¼����: %NGINX_HOME% >> "%LOG_FILE%"

if not exist "%BACKEND_ROOT%" (
    echo [����] ���Ŀ¼δ�ҵ�
    echo [����] ���Ŀ¼δ�ҵ�: %BACKEND_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] ���Ŀ¼����
echo [LOG] ���Ŀ¼����: %BACKEND_ROOT% >> "%LOG_FILE%"

REM Check npm
echo [LOG] ���npm������ >> "%LOG_FILE%"
where npm >nul 2>&1
if errorlevel 1 (
    echo [����] npmδ�ҵ�
    echo [����] npmδ�ҵ���PATH�� >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] npm����
echo [LOG] npm���� >> "%LOG_FILE%"

REM Check cargo
echo [LOG] ���cargo������ >> "%LOG_FILE%"
where cargo >nul 2>&1
if errorlevel 1 (
    echo [����] cargoδ�ҵ�
    echo [����] cargoδ�ҵ���PATH�� >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] cargo����
echo [LOG] cargo���� >> "%LOG_FILE%"

echo.
echo ========== ����ǰ�� ==========
echo [LOG] ��ʼǰ�˹������� >> "%LOG_FILE%"

echo [��Ϣ] �л���ǰ��Ŀ¼...
echo [LOG] �л���ǰ��Ŀ¼: %FRONT_ROOT% >> "%LOG_FILE%"

cd /d "%FRONT_ROOT%"
if errorlevel 1 (
    echo [����] �޷��л���ǰ��Ŀ¼
    echo [����] �޷��л���ǰ��Ŀ¼: %FRONT_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)

echo [��Ϣ] ��ǰĿ¼: %CD%
echo [LOG] ��ǰĿ¼: %CD% >> "%LOG_FILE%"

if not exist "package.json" (
    echo [����] package.jsonδ�ҵ�
    echo [����] package.jsonδ�ҵ�: %CD% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] package.json���ҵ�
echo [LOG] package.json���ҵ�: %CD% >> "%LOG_FILE%"

REM Skip npm install - assume dependencies are already installed
echo.
echo [��Ϣ] ����npm��װ (���������Ѱ�װ)...
echo [LOG] ����npm��װ >> "%LOG_FILE%"

REM Check if node_modules exists
if not exist "node_modules" (
    echo [����] node_modulesĿ¼δ�ҵ�
    echo [����] ��������Ҫ�ֶ����� 'npm install' ����
    echo [����] node_modulesδ�ҵ�: %CD% >> "%LOG_FILE%"
)

echo.
echo [��Ϣ] ������Ŀ (�������Ҫ������)...
echo [����] ִ��: call npm run build
echo [LOG] ��ʼnpm run build: %time% >> "%LOG_FILE%"

REM Use call to prevent terminal from closing
call npm run build
set "build_exit_code=%errorlevel%"

echo [LOG] npm run build�����: %time% �˳���: %build_exit_code% >> "%LOG_FILE%"

REM Force a clear break after build command to prevent output confusion
echo.
echo.
echo.
echo ========================================
echo [����] �����������!
echo [����] �˳���: %build_exit_code%
echo [����] ��������...
echo ========================================
echo.

if %build_exit_code% neq 0 (
    echo [����] npm run buildʧ���˳���: %build_exit_code%
    echo [����] npm run buildʧ���˳���: %build_exit_code% >> "%LOG_FILE%"
    echo [��Ϣ] ����ʧ��, ֹͣ����
    goto :ERROR
)

echo [OK] �����ɹ�
echo [LOG] �����ɹ� >> "%LOG_FILE%"

echo [��Ϣ] ��鹹�����...
set DIST_DIR=%FRONT_ROOT%\dist
echo [����] ����distĿ¼: %DIST_DIR%
echo [LOG] ���distĿ¼: %DIST_DIR% >> "%LOG_FILE%"

if not exist "%DIST_DIR%" (
    echo [����] distĿ¼δ����: %DIST_DIR%
    echo [����] distĿ¼δ����: %DIST_DIR% >> "%LOG_FILE%"
    echo [��Ϣ] �г���ǰĿ¼����:
    dir /b
    dir /b >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] distĿ¼���ҵ�
echo [LOG] distĿ¼���ҵ�: %DIST_DIR% >> "%LOG_FILE%"

echo [��Ϣ] ���distĿ¼����...
if exist "%DIST_DIR%\*" (
    echo [OK] distĿ¼�����ļ�
    echo [LOG] distĿ¼�����ļ� >> "%LOG_FILE%"
    dir "%DIST_DIR%" /b >> "%LOG_FILE%"
) else (
    echo [����] distĿ¼�ƺ�Ϊ��
    echo [����] distĿ¼Ϊ��: %DIST_DIR% >> "%LOG_FILE%"
)

echo.
echo ========== ����Nginx ==========
echo [LOG] ��ʼ����Nginx >> "%LOG_FILE%"

set NGINX_HTML=%NGINX_HOME%\html
set BACKUP_DIR=%NGINX_HOME%\backup_%date:~0,4%%date:~5,2%%date:~8,2%

echo [��Ϣ] ��������...
echo [LOG] ��%NGINX_HTML%�������ݵ�%BACKUP_DIR% >> "%LOG_FILE%"
echo [����] ����Ŀ¼��: %BACKUP_DIR%
echo [����] Nginx HTMLĿ¼: %NGINX_HTML%

if exist "%NGINX_HTML%" (
    echo [����] Nginx HTMLĿ¼����, �������...
    echo [LOG] Nginx HTMLĿ¼����, ������� >> "%LOG_FILE%"
    dir "%NGINX_HTML%" /b >nul 2>&1
    if errorlevel 1 (
        echo [��Ϣ] Nginx HTMLĿ¼Ϊ��, ��������
        echo [LOG] Nginx HTMLĿ¼Ϊ��, �������� >> "%LOG_FILE%"
    ) else (
        echo [����] ��������Ŀ¼...
        echo [LOG] ��������Ŀ¼: %BACKUP_DIR% >> "%LOG_FILE%"
        if not exist "%BACKUP_DIR%" mkdir "%BACKUP_DIR%"
        echo [����] ��ʼxcopy����...
        echo [LOG] ��ʼxcopy���� >> "%LOG_FILE%"
        xcopy "%NGINX_HTML%\*" "%BACKUP_DIR%\" /E /I /H /R /Y
        set "backup_exit=!errorlevel!"
        echo [����] xcopy�˳���: !backup_exit!
        echo [LOG] xcopy�˳���: !backup_exit! >> "%LOG_FILE%"
        if !backup_exit! neq 0 (
            echo [����] ������ɵ��о��� (�˳���: !backup_exit!)
            echo [����] ������ɵ��о��� (�˳���: !backup_exit!) >> "%LOG_FILE%"
        ) else (
            echo [OK] ���ݳɹ�
            echo [LOG] ���ݳɹ� >> "%LOG_FILE%"
        )
        echo [OK] �����Ѵ�����: %BACKUP_DIR%
    )
) else (
    echo [����] Nginx HTMLĿ¼������, ������...
    echo [LOG] Nginx HTMLĿ¼������, ������ >> "%LOG_FILE%"
    mkdir "%NGINX_HTML%"
    echo [OK] Nginx htmlĿ¼�Ѵ���
    echo [LOG] Nginx htmlĿ¼�Ѵ���: %NGINX_HTML% >> "%LOG_FILE%"
)

echo [��Ϣ] ����nginxĿ¼...
echo [LOG] ��ʼnginxĿ¼���� >> "%LOG_FILE%"
echo [����] ����Ŀ¼: %NGINX_HTML%
if exist "%NGINX_HTML%" (
    echo [����] ɾ���ļ�...
    echo [LOG] ��%NGINX_HTML%ɾ���ļ� >> "%LOG_FILE%"
    del /Q "%NGINX_HTML%\*.*" >nul 2>&1
    echo [����] �����Ŀ¼...
    echo [LOG] �����Ŀ¼ >> "%LOG_FILE%"
for /D %%d in ("%NGINX_HTML%\*") do (
        if not "%%~nxd"=="uploads" if not "%%~nxd"=="data.db" (
            echo [����] ɾ��Ŀ¼: %%d
            echo [LOG] ɾ��Ŀ¼: %%d >> "%LOG_FILE%"
            rmdir /S /Q "%%d" >nul 2>&1
        ) else (
            echo [����] ����Ŀ¼: %%d
            echo [LOG] ����Ŀ¼: %%d >> "%LOG_FILE%"
    )
)
    echo [����] Ŀ¼�������
    echo [LOG] Ŀ¼������� >> "%LOG_FILE%"
)
echo [OK] NginxĿ¼������

echo [��Ϣ] ���ļ���%DIST_DIR%�ƶ���%NGINX_HTML%...
echo [LOG] ��ʼ�ļ��ƶ����� >> "%LOG_FILE%"
echo [����] ԴĿ¼: %DIST_DIR%
echo [����] Ŀ��Ŀ¼: %NGINX_HTML%
echo [����] ��ʼrobocopy����...
echo [LOG] robocopy��%DIST_DIR%��%NGINX_HTML% >> "%LOG_FILE%"
robocopy "%DIST_DIR%" "%NGINX_HTML%" /E /MOVE /R:2 /W:1
set "move_exit_code=%errorlevel%"
echo [����] Robocopy����˳���: %move_exit_code%
echo [LOG] Robocopy����˳���: %move_exit_code% >> "%LOG_FILE%"

REM robocopy�˳���: 0-7Ϊ�ɹ�, 8+Ϊ����
if %move_exit_code% GEQ 8 (
    echo [����] �ļ��ƶ�ʧ���˳���: %move_exit_code%
    echo [����] �ļ��ƶ�ʧ���˳���: %move_exit_code% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] �ļ��ƶ��ɹ� (distĿ¼������)
echo [LOG] �ļ��ƶ��ɹ� >> "%LOG_FILE%"

echo [��Ϣ] ��֤����...
echo [LOG] ��֤���� >> "%LOG_FILE%"
if exist "%NGINX_HTML%\index.html" (
    echo [OK] index.html��nginxĿ¼���ҵ�
    echo [LOG] index.html��nginxĿ¼���ҵ� >> "%LOG_FILE%"
) else (
    echo [����] index.htmlδ��nginxĿ¼���ҵ�
    echo [����] index.htmlδ��nginxĿ¼���ҵ� >> "%LOG_FILE%"
)

echo.
echo ========== �������� ==========
echo [LOG] ��ʼ�������� >> "%LOG_FILE%"

echo [��Ϣ] ��ϸNginx��������...
cd /d "%NGINX_HOME%"
echo [LOG] �л���nginxĿ¼: %NGINX_HOME% >> "%LOG_FILE%"

REM ��ϸnginx״̬���
echo [����1] ��鵱ǰNginx����...
echo [LOG] ����1: ��鵱ǰNginx���� >> "%LOG_FILE%"
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [״̬] δ�ҵ�Nginx����
    echo [LOG] δ�ҵ�Nginx���� >> "%LOG_FILE%"
) else (
    echo [״̬] ��ǰNginx����:
    echo [LOG] ��ǰNginx�������ҵ� >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
)

REM ���˿�80�Ƿ�ռ��
echo [����2] ���˿�80ʹ�����...
echo [LOG] ����2: ���˿�80ʹ����� >> "%LOG_FILE%"
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo [״̬] �˿�80����
    echo [LOG] �˿�80���� >> "%LOG_FILE%"
) else (
    echo [״̬] �˿�80��ռ��:
    echo [LOG] �˿�80��ռ�� >> "%LOG_FILE%"
    netstat -ano | findstr ":80 "
    netstat -ano | findstr ":80 " >> "%LOG_FILE%"
)

  REM ֹͣnginx
  echo [����3] ֹͣNginx...
  echo [LOG] ����3: ֹͣNginx >> "%LOG_FILE%"
  
  REM ���ȳ���ʹ���Զ�ֹͣ�ű�
  if exist "stop_nginx_auto.bat" (
      echo [����] ʹ���Զ�ֹͣ�ű�...
      echo [LOG] ʹ���Զ�ֹͣ�ű� >> "%LOG_FILE%"
      call stop_nginx_auto.bat
      set "nginx_stop_exit=!errorlevel!"
      echo [����] stop_nginx_auto.bat�˳���: !nginx_stop_exit!
      echo [LOG] stop_nginx_auto.bat�˳���: !nginx_stop_exit! >> "%LOG_FILE%"
  ) else (
      echo [����] δ�ҵ��Զ�ֹͣ�ű�, ʹ��ֱ������...
      echo [LOG] δ�ҵ��Զ�ֹͣ�ű�, ʹ��ֱ������ >> "%LOG_FILE%"
      
      REM ���nginx�Ƿ���������
      if exist "logs\nginx.pid" (
          echo [����] Nginx��������, ����ֹͣ...
          echo [LOG] Nginx��������, ����ֹͣ >> "%LOG_FILE%"
          nginx.exe -s quit
          echo [LOG] nginx.exe -s quit������ִ�� >> "%LOG_FILE%"
          
          REM �ȴ����Źر�
          echo [����] �ȴ����Źر�...
          echo [LOG] �ȴ����Źر� >> "%LOG_FILE%"
          timeout /t 3 /nobreak >nul
          
          REM ����Ƿ���������
          if exist "logs\nginx.pid" (
              echo [����] Nginx��������, ǿ��ɱ��...
              echo [LOG] Nginx��������, ǿ��ɱ�� >> "%LOG_FILE%"
              taskkill /F /IM "nginx.exe" >nul 2>&1
              if exist "logs\nginx.pid" del "logs\nginx.pid" >nul 2>&1
              echo [LOG] ǿ��ɱ����� >> "%LOG_FILE%"
          ) else (
              echo [OK] Nginx����ֹͣ
              echo [LOG] Nginx����ֹͣ >> "%LOG_FILE%"
          )
      ) else (
          echo [��Ϣ] Nginxδ����
          echo [LOG] Nginxδ���� >> "%LOG_FILE%"
      )
  )

echo [��Ϣ] �ȴ�5��, ȷ��Nginx��ȫֹͣ...
echo [LOG] �ȴ�5��ȷ��Nginxֹͣ >> "%LOG_FILE%"
timeout /t 5 /nobreak >nul

REM ��֤nginx�Ƿ���ֹͣ
echo [����4] ��֤Nginx��ֹͣ...
echo [LOG] ����4: ��֤Nginx��ֹͣ >> "%LOG_FILE%"
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [�ɹ�] ����Nginx������ֹͣ
    echo [LOG] ����Nginx������ֹͣ >> "%LOG_FILE%"
) else (
    echo [����] ĳЩNginx���̿�����������:
    echo [LOG] ĳЩNginx������������ >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
    echo [��Ϣ] ǿ��ɱ��ʣ�����...
    echo [LOG] ǿ��ɱ��ʣ����� >> "%LOG_FILE%"
    taskkill /F /IM "nginx.exe" >nul 2>&1
    timeout /t 2 /nobreak >nul
)

  REM ����nginx
  echo [����5] ����Nginx...
  echo [LOG] ����5: ����Nginx >> "%LOG_FILE%"
  
  REM ���ȳ���ʹ�ò����ض��������ű�
  if exist "start_nginx_deploy.bat" (
      echo [����] ʹ�ò����ض��������ű�...
      echo [LOG] ʹ�ò����ض��������ű� >> "%LOG_FILE%"
      call start_nginx_deploy.bat
      set "nginx_start_exit=!errorlevel!"
      echo [����] start_nginx_deploy.bat�˳���: !nginx_start_exit!
      echo [LOG] start_nginx_deploy.bat�˳���: !nginx_start_exit! >> "%LOG_FILE%"
      
      if !nginx_start_exit! neq 0 (
          echo [����] ���������ű����ش����˳���: !nginx_start_exit!
          echo [LOG] ����: ���������ű����ش����˳���: !nginx_start_exit! >> "%LOG_FILE%"
      ) else (
          echo [OK] Nginx��������������ִ�гɹ�
          echo [LOG] Nginx��������������ִ�гɹ� >> "%LOG_FILE%"
      )
  ) else if exist "start_nginx_auto.bat" (
      echo [����] ʹ���Զ������ű�...
      echo [LOG] ʹ���Զ������ű� >> "%LOG_FILE%"
      call start_nginx_auto.bat
      set "nginx_start_exit=!errorlevel!"
      echo [����] start_nginx_auto.bat�˳���: !nginx_start_exit!
      echo [LOG] start_nginx_auto.bat�˳���: !nginx_start_exit! >> "%LOG_FILE%"
      
      if !nginx_start_exit! neq 0 (
          echo [����] Nginx����ʧ���˳���: !nginx_start_exit!
          echo [LOG] ����: Nginx����ʧ���˳���: !nginx_start_exit! >> "%LOG_FILE%"
      ) else (
          echo [OK] Nginx����������ִ�гɹ�
          echo [LOG] Nginx����������ִ�гɹ� >> "%LOG_FILE%"
      )
  ) else (
      echo [����] δ�ҵ��Զ������ű�, ʹ��ֱ������...
      echo [LOG] δ�ҵ��Զ������ű�, ʹ��ֱ������ >> "%LOG_FILE%"
      
      REM ���nginx�Ƿ�������
      if exist "logs\nginx.pid" (
          echo [��Ϣ] Nginx������
          echo [LOG] Nginx������ >> "%LOG_FILE%"
      ) else (
          echo [����] ����nginx����...
          echo [LOG] ����nginx���� >> "%LOG_FILE%"
          nginx.exe -t
          set "config_test_exit=!errorlevel!"
          echo [LOG] nginx���ò����˳���: !config_test_exit! >> "%LOG_FILE%"
          
          if !config_test_exit! neq 0 (
              echo [����] Nginx���ò���ʧ��
              echo [LOG] Nginx���ò���ʧ�� >> "%LOG_FILE%"
              goto :ERROR
          )
          echo [OK] Nginx������Ч
          echo [LOG] Nginx������Ч >> "%LOG_FILE%"
          
          REM ֱ������nginx
          echo [����] ֱ������nginx...
          echo [LOG] ֱ������nginx >> "%LOG_FILE%"
          start /b "" nginx.exe
          echo [LOG] nginx.exe����������ִ�� >> "%LOG_FILE%"
      )
  )

  echo [��Ϣ] �ȴ�5��, ȷ��Nginx����...
  echo [LOG] �ȴ�5��ȷ��Nginx���� >> "%LOG_FILE%"
  timeout /t 5 /nobreak >nul
  
  REM �������nginx.pid�ļ��Ƿ��Ѵ���
  echo [����] ���nginx.pid�ļ�...
  echo [LOG] ���nginx.pid�ļ� >> "%LOG_FILE%"
  if exist "logs\nginx.pid" (
      echo [����] nginx.pid�ļ����ҵ�
      echo [LOG] nginx.pid�ļ����ҵ� >> "%LOG_FILE%"
      type "logs\nginx.pid" >> "%LOG_FILE%"
  ) else (
      echo [����] nginx.pid�ļ�δ�ҵ�
      echo [LOG] nginx.pid�ļ�δ�ҵ� >> "%LOG_FILE%"
  )
  
  REM �ȴ�һ��ʱ��, ȷ��������ȫ����
  echo [����] �ȴ�����3��, ȷ�������ȶ�...
  echo [LOG] �ȴ�����3��ȷ�������ȶ� >> "%LOG_FILE%"
  timeout /t 3 /nobreak >nul

REM ��ϸ��֤nginx����
echo [����6] ��֤Nginx����...
echo [LOG] ����6: ��֤Nginx���� >> "%LOG_FILE%"

REM ���ȼ��nginx.pid�Ƿ����
if exist "logs\nginx.pid" (
    echo [����] nginx.pid�ļ�����, ������...
    echo [LOG] nginx.pid�ļ�����, ������ >> "%LOG_FILE%"
) else (
    echo [����] nginx.pid�ļ�δ�ҵ�, ���������̼��...
    echo [LOG] nginx.pid�ļ�δ�ҵ�, ���������̼�� >> "%LOG_FILE%"
)

tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [����] Nginx����δ��������
    echo [LOG] ����: Nginx����δ�������� >> "%LOG_FILE%"
    echo [��Ϣ] �ȴ�����5�벢�ٴμ��...
    echo [LOG] �ȴ�����5�벢�ٴμ�� >> "%LOG_FILE%"
    timeout /t 5 /nobreak >nul
    
    tasklist | findstr /I "nginx.exe" >nul 2>&1
    if errorlevel 1 (
        echo [����] Nginx������δ�ҵ�
        echo [LOG] ����: Nginx������δ�ҵ� >> "%LOG_FILE%"
        echo [��Ϣ] ��������־...
        if exist "logs\error.log" (
            echo [����] error.log���5��:
            echo [LOG] error.log���5��: >> "%LOG_FILE%"
            powershell -Command "Get-Content 'logs\error.log' -Tail 5"
            powershell -Command "Get-Content 'logs\error.log' -Tail 5" >> "%LOG_FILE%"
        )
        echo [��Ϣ] Nginx��������ʧ��, ����������...
        echo [LOG] Nginx��������ʧ��, ���������� >> "%LOG_FILE%"
    ) else (
        echo [�ɹ�] Nginx�������ҵ�
        echo [LOG] Nginx�������ҵ� >> "%LOG_FILE%"
        tasklist | findstr /I "nginx.exe"
        tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
    )
) else (
    echo [�ɹ�] Nginx������������:
    echo [LOG] Nginx������������ >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
)

REM ���˿�80
echo [����7] ��֤�˿�80�Ƿ��Ծ...
echo [LOG] ����7: ��֤�˿�80�Ƿ��Ծ >> "%LOG_FILE%"
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo [����] �˿�80δ���Ϊ��Ծ
    echo [LOG] ����: �˿�80δ���Ϊ��Ծ >> "%LOG_FILE%"
) else (
    echo [�ɹ�] �˿�80��Ծ:
    echo [LOG] �˿�80��Ծ >> "%LOG_FILE%"
    netstat -ano | findstr ":80 "
    netstat -ano | findstr ":80 " >> "%LOG_FILE%"
)

REM ����HTTP����
echo [����8] ����HTTP����...
echo [LOG] ����8: ����HTTP���� >> "%LOG_FILE%"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost' -TimeoutSec 5; Write-Host '[�ɹ�] HTTP���Ӳ���ͨ��' } catch { Write-Host '[����] HTTP���Ӳ���ʧ��:' $_.Exception.Message }"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost' -TimeoutSec 5; '[LOG] HTTP���Ӳ���ͨ��' } catch { '[LOG] HTTP���Ӳ���ʧ��: ' + $_.Exception.Message }" >> "%LOG_FILE%"

echo.
echo ========== ������� ==========
echo [LOG] ��ʼ����������� >> "%LOG_FILE%"

echo [��Ϣ] �����˷���...
cd /d "%BACKEND_ROOT%"
echo [LOG] �л������Ŀ¼: %BACKEND_ROOT% >> "%LOG_FILE%"

REM ��鵱ǰ��˽���
echo [����1] ��鵱ǰ��˽���...
echo [LOG] ����1: ��鵱ǰ��˽��� >> "%LOG_FILE%"
tasklist | findstr /I "rope-manager-backend.exe" >nul 2>&1
if errorlevel 1 (
    echo [״̬] δ�ҵ���˽���
    echo [LOG] δ�ҵ���˽��� >> "%LOG_FILE%"
) else (
    echo [״̬] ��ǰ��˽���:
    echo [LOG] ��ǰ��˽������ҵ� >> "%LOG_FILE%"
    tasklist | findstr /I "rope-manager-backend.exe"
    tasklist | findstr /I "rope-manager-backend.exe" >> "%LOG_FILE%"
)

REM ����˶˿� (���������15201)
echo [����2] ����˶˿�15201ʹ�����...
echo [LOG] ����2: ����˶˿�15201ʹ����� >> "%LOG_FILE%"
netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo [״̬] �˿�15201����
    echo [LOG] �˿�15201���� >> "%LOG_FILE%"
) else (
    echo [״̬] �˿�15201��ռ��:
    echo [LOG] �˿�15201��ռ�� >> "%LOG_FILE%"
    netstat -ano | findstr ":15201 "
    netstat -ano | findstr ":15201 " >> "%LOG_FILE%"
)

REM ��ֹ���к�˽���
echo [����3] ��ֹ���к�˽���...
echo [LOG] ����3: ��ֹ���к�˽��� >> "%LOG_FILE%"
taskkill /F /IM "rope-manager-backend.exe" >nul 2>&1
taskkill /F /IM "cargo.exe" >nul 2>&1
timeout /t 3 /nobreak >nul

REM �����º�˽���
echo [����4] �����º�˽���...
echo [LOG] ����4: �����º�˽��� >> "%LOG_FILE%"
echo [����] ����: start "Rope Manager Backend" cmd /k "cargo run"
start "Rope Manager Backend" cmd /k "cargo run"
echo [LOG] �������������ִ�� >> "%LOG_FILE%"

echo [��Ϣ] �ȴ�10��, ȷ����˳�ʼ��...
echo [LOG] �ȴ�10��ȷ����˳�ʼ�� >> "%LOG_FILE%"
timeout /t 10 /nobreak >nul

REM ��֤�������
echo [����5] ��֤�������...
echo [LOG] ����5: ��֤������� >> "%LOG_FILE%"
tasklist | findstr /I "cargo.exe" >nul 2>&1
if errorlevel 1 (
    echo [����] Cargo����δ��⵽
    echo [LOG] ����: Cargo����δ��⵽ >> "%LOG_FILE%"
) else (
    echo [�ɹ�] Cargo������������:
    echo [LOG] Cargo������������ >> "%LOG_FILE%"
    tasklist | findstr /I "cargo.exe"
    tasklist | findstr /I "cargo.exe" >> "%LOG_FILE%"
)

REM ����˶˿��Ƿ��Ծ
echo [����6] ����˶˿�15201�Ƿ��Ծ...
echo [LOG] ����6: ����˶˿�15201�Ƿ��Ծ >> "%LOG_FILE%"
netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo [����] ��˶˿�15201δ��Ծ (������������)
    echo [LOG] ��˶˿�15201δ��Ծ >> "%LOG_FILE%"
) else (
    echo [�ɹ�] ��˶˿�15201��Ծ:
    echo [LOG] ��˶˿�15201��Ծ >> "%LOG_FILE%"
    netstat -ano | findstr ":15201 "
    netstat -ano | findstr ":15201 " >> "%LOG_FILE%"
)

REM ���Ժ��API����
echo [����7] ���Ժ��API����...
echo [LOG] ����7: ���Ժ��API���� >> "%LOG_FILE%"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost:15201' -TimeoutSec 5; Write-Host '[�ɹ�] ���API���Ӳ���ͨ��' } catch { Write-Host '[��Ϣ] ��˿�������������APIδ׼����:' $_.Exception.Message }"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost:15201' -TimeoutSec 5; '[LOG] ���API���Ӳ���ͨ��' } catch { '[LOG] ���API���Ӳ���ʧ��: ' + $_.Exception.Message }" >> "%LOG_FILE%"

echo.
echo ========== ����״̬�ܽ� ==========
echo [LOG] ��������״̬�ܽ� >> "%LOG_FILE%"

echo [��Ϣ] �����ܽ�:
echo ----------------------------------------
echo ǰ�˹���: �����
echo �ļ�����: �����
echo Nginx״̬: 
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo   [����] δ����
    echo [LOG] ����״̬ - Nginx: δ���� >> "%LOG_FILE%"
) else (
    echo   [�ɹ�] ������
    echo [LOG] ����״̬ - Nginx: ������ >> "%LOG_FILE%"
)

echo ���״̬:
tasklist | findstr /I "cargo.exe" >nul 2>&1
if errorlevel 1 (
    echo   [����] δ��⵽
    echo [LOG] ����״̬ - ���: δ��⵽ >> "%LOG_FILE%"
) else (
    echo   [�ɹ�] ������
    echo [LOG] ����״̬ - ���: ������ >> "%LOG_FILE%"
)

echo �˿�״̬:
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo   Port 80 (Nginx): δ��Ծ
    echo [LOG] ����״̬ - Port 80: δ��Ծ >> "%LOG_FILE%"
) else (
    echo   Port 80 (Nginx): ��Ծ
    echo [LOG] ����״̬ - Port 80: ��Ծ >> "%LOG_FILE%"
)

netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo   Port 15201 (Backend): δ��Ծ
    echo [LOG] ����״̬ - Port 15201: δ��Ծ >> "%LOG_FILE%"
) else (
    echo   Port 15201 (Backend): ��Ծ
    echo [LOG] ����״̬ - Port 15201: ��Ծ >> "%LOG_FILE%"
)

echo ----------------------------------------

echo.
echo ========== �ɹ� ==========
echo [�ɹ�] ������������!
echo [��Ϣ] ����λ��: %BACKUP_DIR%
echo [��Ϣ] ��־�ļ�: %LOG_FILE%
echo [��Ϣ] �����Է���Ӧ�ó���: http://localhost
echo [��Ϣ] ���APIӦ����: http://localhost:15201
echo.
echo [LOG] ����ɹ������: %time% >> "%LOG_FILE%"
echo ========== ����ű����� ========== >> "%LOG_FILE%"
goto :END

:ERROR
echo.
echo ========== ���� ==========
echo [����] ����ʧ��!
echo [��Ϣ] ������־�ļ�: %LOG_FILE%
echo.
echo [LOG] ����: ����ʧ����: %time% >> "%LOG_FILE%"
echo [LOG] �ű��������ֹ >> "%LOG_FILE%"
if exist "%BACKUP_DIR%" (
    echo [��Ϣ] ���ݿ���: %BACKUP_DIR%
    echo [��Ϣ] Ҫ�ָ�����, ������: xcopy "%BACKUP_DIR%\*" "%NGINX_HTML%\" /E /I /Q
    echo [LOG] ���ݿ���: %BACKUP_DIR% >> "%LOG_FILE%"
)
echo ========== ����ű����� (����) ========== >> "%LOG_FILE%"

:END
echo.
echo [��Ϣ] �ű�ִ�����
echo [��Ϣ] ��־�ļ��ѱ��浽: %LOG_FILE%
echo [��Ϣ] ��������˳�...
echo [LOG] �ű�ִ�������: %time% >> "%LOG_FILE%"
pause