// written by carter. 20250330
// "who ever thought the greatest rapper would be from coincidence?"
namespace Lvl64Exporter;

using SM64Lib;
using SM64Lib.Levels;

class Program {
  private const string LevelTablePath = "leveltable.json";
  private static bool LoadingLevels = true;

  static void Main(string[] args) {
    if (args.Length < 2) {
      throw new Exception("whoops! you didn't specify the correct number of arguments. please refer to the README for more information.");
    }

    var dir = args[1];

    if (Directory.Exists(dir)) {
      throw new Exception($"whoops! the directory {dir} already exists. please refer to the README for more information.");
    }

    Directory.CreateDirectory(dir);

    Console.WriteLine("loading rom...");
    var path = args[0];
    var rm = new RomManager(path);
    
    if (!File.Exists(LevelTablePath)) {
      throw new Exception("whoops! you don't have the leveltable.json file next to the binary. please refer to the README for more information.");
    }
    rm.LevelInfoData.ReadFromFile(LevelTablePath);
    
    rm.AfterRomLoaded += DoneLoadingLevels;
    rm.LoadRom();
    while (LoadingLevels) {}
    
    foreach (Level level in rm.Levels) {
      Console.WriteLine("exporting level {0}", level.LevelID);
      var name = rm.LevelInfoData.FirstOrDefault(n => n.ID == level.LevelID).Name;
      var export = new LevelExport(level);
      export.WriteToFile(Path.Join(dir, $"{name}.lvl64"), System.IO.Compression.CompressionLevel.Optimal);
    }
    
    Console.WriteLine("finished!");
  }

  static void DoneLoadingLevels(SM64Lib.RomManager rm, EventArgs e) {
    LoadingLevels = false;
  }
}
