use glam::DVec3;

pub fn vec3_coordinate_system(v1:DVec3,v2:&mut DVec3,v3:&mut DVec3){
    if v1.x.abs()>v1.y.abs(){
        *v2=DVec3::new(-v1.z, 0.0, v1.x)
        /(v1.x*v1.x+v1.z*v1.z).sqrt();
    }else{
        *v2=DVec3::new(0.0, v1.z, -v1.y)
        /(v1.y*v1.y+v1.z+v1.z).sqrt();
    }
    *v3=v1.cross(*v2);
}