import { invoke } from "@tauri-apps/api/core";

const TestWindow = () => {
  const handleClose = async () => {
    try {
      await invoke("close_window");
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  return (
    <div className="fixed inset-0 bg-white/15 rounded-xl backdrop-blur-sm flex h-full">
      <button
        onClick={handleClose}
        className="absolute top-2 right-2 bg-black/15 backdrop-blur-sm rounded-full p-2"
      >
        X
      </button>
      <div className="flex-1 flex items-center justify-center">
        <div className="text-2xl font-bold">TestWindow</div>
      </div>
    </div>
  );
};

export default TestWindow;
