using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace passguard
{
    public partial class MainWindow : Form
    {
        public MainWindow()
        {
            InitializeComponent();
            this.Load += MainWindow_Load;
        }

        private void MainWindow_Load(object sender, EventArgs e)
        {
            MessageBox.Show(Lib.CreateUser("admin", "test"));
        }
    }
}
