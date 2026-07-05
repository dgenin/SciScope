# Architectural RFC: Real-Time Microscopy Application

## 1. Introduction
This document outlines the proposed architecture for a high-performance scientific microscopy application. The system is constrained by strict performance requirements:
*   **Target Frame Rates:** Stable 60 FPS at 1080p; up to 4K streams at 30 FPS.
*   **Latency Mandate:** Frame processing pipeline latency must stay below 16ms to prevent visual tearing or UI stuttering.
*   **Memory Management:** Zero-copy memory buffers, memory-mapped I/O, and no Garbage Collection (GC) pauses during the critical rendering loop.

## 2. Language Ecosystem Evaluation

### C++
*   **Pros:** The undisputed industry standard for high-performance DSP and computer vision. Offers massive ecosystems (OpenCV, Qt), zero-overhead abstractions, and explicit, fine-grained memory control.
*   **Cons:** Manual memory management and lack of strict safety guarantees can lead to memory leaks, use-after-free errors, and undefined behavior. The build ecosystem (CMake) can be complex and brittle.

### Rust
*   **Pros:** Guarantees memory and thread safety without a garbage collector. "Fearless concurrency" perfectly aligns with our multithreading needs. Exceptional package manager (Cargo). Performance is on par with, or occasionally exceeds, C/C++.
*   **Cons:** The ecosystem for UI and highly specialized CV is slightly less mature than C++, though rapidly expanding. The borrow checker introduces a steeper learning curve.

### Zig
*   **Pros:** Minimalist language design, excellent C interoperability, and explicit allocation (no hidden control flow or hidden allocations). Very fast compilation and serves as an excellent C/C++ drop-in compiler.
*   **Cons:** Very nascent ecosystem. The language and standard library are still subject to breaking changes. Lacks mature, off-the-shelf UI and DSP frameworks compared to C++ and Rust.

**Proposed Tech Stack: Rust**
*Reasoning:* Rust provides the performance of C++ while eliminating entire classes of memory and concurrency bugs. Its strict borrow checker ensures that data races in our multithreading pipeline are caught at compile-time. We can still utilize C/C++ libraries via FFI where absolutely necessary, but Rust's native ecosystem (e.g., `rayon` for data parallelism) will provide a robust foundation.

## 3. UI Framework Evaluation

### Qt (via C++ or Rust bindings like CXX-Qt)
*   **Pros:** Extremely mature, feature-rich, and battle-tested. Hardware-accelerated rendering via Qt Quick/QML.
*   **Cons:** Very heavy footprint, complex build pipeline, and bridging idiomatic Rust with Qt's C++ object model can be cumbersome and introduce overhead.

### Slint
*   **Pros:** A modern, lightweight, declarative UI framework built in Rust (with C++ support). Compiles UI down to native code, resulting in an exceptionally low memory footprint and fast startup times.
*   **Cons:** Smaller widget ecosystem compared to Qt; less mature.

### Native Bindings (Win32, Cocoa, GTK)
*   **Pros:** Absolute lowest overhead and true native integration.
*   **Cons:** Massive development effort required to maintain separate UI codebases for Windows, macOS, and Linux.

**Proposed UI Framework: Slint**
*Reasoning:* Slint perfectly complements our choice of Rust. It avoids the bloat of Qt while providing hardware-accelerated rendering. Its declarative nature combined with Rust's backend allows us to build a snappy, low-overhead UI that won't compete for resources with the DSP pipeline.

## 4. Multithreading Strategy

To guarantee < 16ms latency, the camera ingest loop must be strictly decoupled from the UI rendering thread. We propose a three-tiered thread architecture:

1.  **Camera Ingest & Hardware Control (High Priority Thread):**
    *   Dedicated exclusively to interfacing with the hardware (e.g., UVC, V4L2).
    *   Reads raw frames directly into memory-mapped buffers.
    *   Never blocks on UI or heavy processing.
2.  **DSP & Image Processing Pipeline (Worker Pool):**
    *   A thread pool (e.g., utilizing `rayon`) that receives buffer handles from the ingest thread.
    *   Applies algorithmic transformations: debayering, white balance, color correction.
3.  **UI & Rendering (Main Thread):**
    *   Runs the Slint event loop.
    *   Receives fully processed buffer handles and uploads them to the GPU (e.g., via OpenGL/Vulkan texture streaming) for display.

*Synchronization:* Threads will communicate using lock-free data structures (e.g., `crossbeam-channel` in Rust) to pass buffer ownership indices, completely avoiding mutex contention on the fast path. If the DSP or UI thread falls behind, the Camera Ingest thread will employ a "drop oldest" strategy to maintain hardware synchronization and real-time latency.

## 5. Zero-Copy Frame Buffer Architecture

To achieve 60 FPS without GC stutters or allocation overhead, we will implement a static Ring Buffer Pool:

1.  **Pre-allocation:** Upon initialization, a fixed-size pool of large memory buffers (e.g., 3 to 5 frames worth of 4K memory) is allocated. No dynamic allocations (`malloc`, `new`, or Rust `Vec` resizing) will occur during the streaming loop.
2.  **Direct Memory Access (DMA):** Hardware drivers will be configured to write incoming frame data directly into these pre-allocated buffers (memory-mapped I/O).
3.  **Ownership Passing:**
    *   The Ingest thread acquires an index to an empty buffer.
    *   Once filled, it passes the *index* (not the data) to the DSP thread.
    *   The DSP thread processes the data in-place if possible (or outputs to a secondary pre-allocated pool).
    *   The index is passed to the UI thread.
    *   After GPU texture upload, the UI thread returns the index to the "empty" queue.
4.  **Zero-Copy Guarantee:** At no point is the raw pixel data copied from one CPU memory space to another. The only "copy" is the unavoidable upload from System RAM to VRAM for GPU rendering.

## 6. Conclusion
By leveraging **Rust** for memory-safe concurrency, **Slint** for lightweight declarative UI, and a strict **Zero-Copy Ring Buffer**, we can confidently meet the 60 FPS @ 1080p and <16ms latency mandates without the risk of GC pauses or UI blocking.
