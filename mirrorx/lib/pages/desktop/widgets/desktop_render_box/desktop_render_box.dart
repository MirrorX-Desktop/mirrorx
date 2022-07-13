import 'dart:developer';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box_scrollbar.dart';

class DesktopRenderBox extends StatefulWidget {
  const DesktopRenderBox({
    Key? key,
    required this.model,
    required this.width,
    required this.height,
  }) : super(key: key);

  final DesktopModel model;
  final int width;
  final int height;

  @override
  _DesktopRenderBoxState createState() => _DesktopRenderBoxState();
}

class _DesktopRenderBoxState extends State<DesktopRenderBox> {
  double offsetY = 0.0;
  double offsetX = 0.0;

  @override
  void initState() {
    super.initState();
    log("initial width: ${widget.width} height: ${widget.height}");
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: offsetY,
          left: offsetX,
          width: widget.width.floorToDouble(),
          height: widget.height.floorToDouble(),
          child: _buildTexture(),
        ),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: widget.height.floorToDouble(),
            axis: Axis.vertical,
            trunkWidth: constraints.maxHeight,
            onScroll: (offset) {
              setState(() {
                offsetY = -offset;
                if ((offsetY + constraints.maxHeight) > widget.height) {
                  offsetY = widget.height - constraints.maxHeight;
                }
              });
            },
          );
        }),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: widget.width.floorToDouble(),
            axis: Axis.horizontal,
            trunkWidth: constraints.maxWidth,
            onScroll: (offset) {
              setState(() {
                offsetX = -offset;
                if ((offsetX + constraints.maxWidth) > widget.width) {
                  offsetX = widget.width - constraints.maxWidth;
                }
              });
            },
          );
        })
      ],
    );
  }

  Widget _buildTexture() {
    return Listener(
      behavior: HitTestBehavior.opaque,
      onPointerDown: _handlePointerDown,
      onPointerUp: _handlePointerUp,
      onPointerHover: _handlePointerHover,
      onPointerSignal: _handlePointerSignal,
      child: RepaintBoundary(
        child: Container(
          color: Colors.black,
          child: Center(
            child: AspectRatio(
              aspectRatio: widget.width.toDouble() / widget.height.toDouble(),
              child: Texture(
                textureId: widget.model.textureID,
                freeze: true,
                filterQuality: FilterQuality.medium,
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _handlePointerDown(PointerDownEvent event) {
    log("pointer down ${event.buttons}");

    var mouseKey = MouseKey.None;

    switch (event.buttons) {
      case kPrimaryMouseButton:
        mouseKey = MouseKey.Left;
        break;
      case kSecondaryMouseButton:
        mouseKey = MouseKey.Right;
        break;
      case kMiddleMouseButton:
        mouseKey = MouseKey.Wheel;
    }

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.down(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );
  }

  void _handlePointerUp(PointerUpEvent event) {
    var mouseKey = MouseKey.None;

    switch (event.buttons) {
      case kPrimaryMouseButton:
        mouseKey = MouseKey.Left;
        break;
      case kSecondaryMouseButton:
        mouseKey = MouseKey.Right;
        break;
      case kMiddleMouseButton:
        mouseKey = MouseKey.Wheel;
    }

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.up(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );
  }

  void _handlePointerHover(PointerHoverEvent event) {
    var mouseKey = MouseKey.None;

    if (event.down) {
      switch (event.buttons) {
        case kPrimaryMouseButton:
          mouseKey = MouseKey.Left;
          break;
        case kSecondaryMouseButton:
          mouseKey = MouseKey.Right;
          break;
        case kMiddleMouseButton:
          mouseKey = MouseKey.Wheel;
      }
    }

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.move(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      MirrorXCoreSDK.instance.endpointMouseEvent(
        remoteDeviceId: widget.model.remoteDeviceID,
        event: MouseEvent.scrollWheel(event.scrollDelta.dy),
        x: event.localPosition.dx,
        y: event.localPosition.dy,
      );
    }
  }
}
