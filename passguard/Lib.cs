using System;
using System.Runtime.InteropServices;

public class Lib
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Parameters
    {
        public string methodName;
        public string username;
        public string password;
    }

    private const string LIBPATH = "passguard_api.dll";
    [DllImport(LIBPATH, EntryPoint = "entrypoint")]
    private static extern IntPtr CallApi(IntPtr parameters);

    public static string CreateUser(string username, string password)
    {
        Parameters parameters = new Parameters
        {
            methodName = "createUser",
            username = username,
            password = password
        };
        IntPtr newParameters = Marshal.AllocHGlobal(Marshal.SizeOf<Parameters>());
        Marshal.StructureToPtr<Parameters>(parameters, newParameters, false);
        IntPtr response = CallApi(newParameters);
        return Marshal.PtrToStringAuto(response);
    }
}