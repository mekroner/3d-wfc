# Batch glTF 2.0 Exporter for Blender
# Copyright © 2020 John Croisant
#
# Permission is hereby granted, free of charge, to any person
# obtaining a copy of this software and associated documentation files
# (the “Software”), to deal in the Software without restriction,
# including without limitation the rights to use, copy, modify, merge,
# publish, distribute, sublicense, and/or sell copies of the Software,
# and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
# NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
# BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
# ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.


# See the README or https://gitlab.com/jcroisant/batch_export_gltf
# for usage instructions.


import bpy
import os
from mathutils import Vector, Quaternion


bl_info = {
    'name': 'Batch Export glTF 2.0',
    'description': 'Export multiple files using glTF 2.0 exporter',
    'author': 'John Croisant',
    'version': (1, 1),
    'blender': (2, 80, 0),
    'category': 'Import-Export',
    'location': 'File > Export > Batch glTF 2.0 (.glb/.gltf)',
    'tracker_url': 'https://gitlab.com/jcroisant/batch_export_gltf/issues'
}


class BatchGltfExport(bpy.types.Operator):
    """Batch export every Collection that starts with 'Export:'"""

    bl_idname = 'export.batch_export_gltf2'
    bl_label = 'Batch glTF 2.0 (.glb/.gltf)'
    bl_options = {'REGISTER'}

    def execute(self, context):
        
        # Directory this blend file is saved in.
        basedir = os.path.dirname(bpy.data.filepath)
        if not basedir:
            raise Exception('Blend file is not saved')
        
        # create export dir 
        export_dir_name = "export"
        export_dir = os.path.join(basedir, export_dir_name)
        
        if not os.path.exists(export_dir):
            os.makedirs(export_dir)
            print("created export folder")
        

        # Remember old mode, and switch to object mode.
        old_mode = context.mode
        bpy.ops.object.mode_set(mode='OBJECT')

        # Remember old active and selective objects,
        # then deselect all objects.
        old_active_objects = context.view_layer.objects.active
        old_selection = context.selected_objects
        bpy.ops.object.select_all(action='DESELECT')

        # Remember old transformations of objects with
        # export_reset_loc, export_reset_rot, or export_reset_scale
        # custom properties.
        old_locs = {}
        old_rots = {}
        old_scales = {}

        # Get saved glTF 2.0 exporter settings if they exist.
        settings = context.scene.get('glTF2ExportSettings', {})

        # Remember which collections have been exported.
        exported = []

        for coll in context.scene.collection.children:
            if not coll.name.startswith("Export:"):
                continue

            # Temporarily select all the objects in this collection,
            # excluding any with the 'export_exclude' custom property,
            # so the exporter will only export them.
            for obj in coll.objects:
                if obj.data and not ('export_exclude' in obj.data):
                    obj.select_set(True)

                # Remember original location and move to origin if
                # 'export_reset_loc' custom property is set.
                # if obj.data and ('export_reset_loc' in obj.data):
                old_locs[obj] = obj.location.to_tuple()
                obj.location = Vector((0, 0, 0))

                # Remember original rotation and reset rotation if
                # 'export_reset_rot' custom property is set.
                # if obj.data and ('export_reset_rot' in obj.data):
                mode = obj.rotation_mode
                obj.rotation_mode = 'QUATERNION'
                q = obj.rotation_quaternion
                old_rots[obj] = ((q.w, q.x, q.y, q.z), mode)
                obj.rotation_quaternion = Quaternion((1, 0, 0, 0))

                # Remember original scale and reset scale if
                # 'export_reset_scale' custom property is set.
                # if obj.data and ('export_reset_scale' in obj.data):
                #old_scales[obj] = obj.scale.to_tuple()
                #obj.scale = Vector((1, 1, 1))

            # Derive the export file name from the collection name.
            export_name = str.replace(coll.name, 'Export:', '').strip()
            filepath = os.path.join(export_dir, export_name)

            self.export_to_file(filepath, settings)
            exported.append(export_name)

            # Deselect the objects, and restore old transformations
            # if necessary, to return to a clean state.
            for obj in coll.objects:
                obj.select_set(False)
                if obj in old_locs:
                    obj.location = Vector(old_locs.pop(obj))
                if obj in old_rots:
                    (q, mode) = old_rots.pop(obj)
                    obj.rotation_quaternion = Quaternion(q)
                    obj.rotation_mode = mode
                if obj in old_scales:
                    obj.scale = Vector(old_scales.pop(obj))

        # Restore previous active and selected objects.
        context.view_layer.objects.active = old_active_objects
        for obj in old_selection:
            obj.select_set(True)

        # Try to return to previous mode.
        if old_mode.startswith('EDIT_'):
            bpy.ops.object.editmode_toggle()
        else:
            bpy.ops.object.mode_set(mode=old_mode)

        # Tell the user which collections were exported, or give a
        # warning to help them.
        if exported:
            self.report({'INFO'}, 'Exported ' + ', '.join(exported))
        else:
            self.report(
                {'WARNING'},
                'Nothing exported. '
                'Name a collection e.g. "Export: player".'
            )

        return {'FINISHED'}

    def export_to_file(self, filepath, settings):
        # Override some default settings.
        settings = {
            **settings,
            **{
                'filepath': filepath,
                'use_selection': True,
                'check_existing': False,
                'will_save_settings': False,
            }
        }
        bpy.ops.export_scene.gltf(**settings)


def menu_func(self, context):
    self.layout.operator(BatchGltfExport.bl_idname)


def register():
    bpy.utils.register_class(BatchGltfExport)
    bpy.types.TOPBAR_MT_file_export.append(menu_func)


def unregister():
    bpy.utils.unregister_class(BatchGltfExport)
    bpy.types.TOPBAR_MT_file_export.remove(menu_func)


if __name__ == "__main__":
    register()